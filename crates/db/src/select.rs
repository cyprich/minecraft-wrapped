use std::collections::HashMap;

use anyhow::Context;
use log::{trace, warn};
use shared::{PlayerSnapshot, StatCategory};
use sqlx::{query_as, query_scalar};

use crate::Manager;
use crate::models::{PlayerRow, StatRow};

pub async fn select_player_snapshots(manager: &Manager) -> anyhow::Result<Vec<PlayerSnapshot>> {
    // this will be needed later, but there is no need to continue if this fails
    let players = query_as!(PlayerRow, "select * from players")
        .fetch_all(&manager.pool)
        .await?;
    // key: id, value: uuid
    let mut player_uuid_from_id = HashMap::new();
    players.into_iter().for_each(|p| {
        player_uuid_from_id.entry(p.id).or_insert(p.uuid);
    });
    trace!("Player UUIDs: {:?}", player_uuid_from_id);

    let mut stats = Vec::new();
    const CHUNK_SIZE: i64 = 2048;
    let mut offset = 0i64;

    let count = query_scalar!("select count(*) from stats")
        .fetch_one(&manager.pool)
        .await?
        .context("Failed to select count of stats")?;
    trace!("About to load {} stats", count);

    while CHUNK_SIZE + offset < count {
        let stats_chunk = query_as!(
            StatRow,
            "select * from stats limit $1 offset $2",
            CHUNK_SIZE,
            offset
        )
        .fetch_all(&manager.pool)
        .await?;
        trace!("Got {} to {} stats", offset, offset + CHUNK_SIZE);

        stats.push(stats_chunk);
        offset += CHUNK_SIZE;
    }
    trace!("Loading stats done");
    let stats = stats.into_iter().flatten();

    // key: (player_id, datetime), value = Vec<Stat { category, name, value }>
    let mut stat_map = HashMap::new();

    for stat in stats {
        let category = match StatCategory::try_from_str(&stat.category) {
            Some(val) => val,
            None => {
                warn!("Unknown category: {}", &stat.category);
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
            let uuid = match player_uuid_from_id.get(&player_id) {
                Some(val) => val,
                None => {
                    warn!("Player ID {} not found", player_id);
                    return None;
                }
            };
            Some(PlayerSnapshot::from_uuid(*uuid, stats, datetime))
        })
        .collect::<Vec<_>>();

    trace!("Returning {} PlayerSnapshots", result.len());

    Ok(result)
}
