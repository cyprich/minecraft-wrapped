use std::collections::HashMap;

use anyhow::Context;
use log::{trace, warn};
use shared::{Player, PlayerSnapshot, StatCategory};
use sqlx::{query_as, query_scalar};

use crate::models::{PlayerRow, PlayerUuidRow, StatRow};
use crate::{Manager, SELECT_CHUNK_SIZE};

pub async fn select_players(manager: &Manager) -> anyhow::Result<Vec<Player>> {
    let players = query_as!(PlayerRow, "select * from players")
        .fetch_all(&manager.pool)
        .await?;

    let uuids = query_as!(
        PlayerUuidRow,
        "select * from player_uuids where player_id is not null"
    )
    .fetch_all(&manager.pool)
    .await?;

    let mut players = players
        .into_iter()
        .map(|p| p.into_player())
        .collect::<Vec<_>>();

    for p in &mut players {
        for u in &uuids {
            if u.player_id.unwrap() == p.id {
                p.uuids.push(u.uuid)
            }
        }
    }

    Ok(players)
}

/// select player snapshots
pub async fn select_player_snapshots(manager: &Manager) -> anyhow::Result<Vec<PlayerSnapshot>> {
    let mut stats = Vec::new();
    let mut offset = 0i64;

    let count = query_scalar!("select count(*) from stats")
        .fetch_one(&manager.pool)
        .await?
        .context("Failed to select count of stats")?;
    trace!("About to load {} stats", count);

    while SELECT_CHUNK_SIZE + offset < count {
        let stats_chunk = query_as!(
            StatRow,
            "select * from stats limit $1 offset $2",
            SELECT_CHUNK_SIZE,
            offset
        )
        .fetch_all(&manager.pool)
        .await?;
        trace!("Got {} to {} stats", offset, offset + SELECT_CHUNK_SIZE);

        stats.push(stats_chunk);
        offset += SELECT_CHUNK_SIZE;
    }
    trace!("Loading stats done");
    let stats = stats.into_iter().flatten();

    // group by player id and date
    // key: (player_id, datetime), value = Vec<Stat { category, name, value }>
    let mut stat_map = HashMap::new();

    for stat in stats {
        let category = match StatCategory::try_from_str(&stat.category) {
            Some(val) => val,
            None => {
                warn!("Unknown category: {}", stat.category);
                continue;
            }
        };

        stat_map
            .entry((stat.player_id, stat.timestamp))
            .or_insert(Vec::new())
            .push(shared::StatValue::new(
                category,
                stat.name,
                stat.value as u32,
            ));
    }

    let result = stat_map
        .into_iter()
        .filter_map(|((player_id, datetime), stats)| {
            let result = PlayerSnapshot::new(player_id, stats, datetime).ok()?;
            Some(result)
        })
        .collect::<Vec<_>>();

    trace!("Returning {} PlayerSnapshots", result.len());

    Ok(result)
}
