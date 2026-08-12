use std::collections::HashMap;

use anyhow::Context;
use log::{error, info};
use shared::PlayerSnapshot;
use sqlx::{query_as, query_scalar, types::Uuid};

use crate::{Manager, models::Player};

pub async fn insert_player(manager: &Manager, name: &str, uuid: &Uuid) -> anyhow::Result<i32> {
    let id = query_scalar!(
        "insert into players (name, uuid) values ($1, $2) returning id",
        name,
        uuid
    )
    .fetch_one(&manager.pool)
    .await?;

    Ok(id)
}

// TODO: make the parameter `Vec<&PlayerStats>`?
pub async fn insert_player_snapshots(
    manager: &Manager,
    stats: Vec<PlayerSnapshot>,
) -> anyhow::Result<()> {
    let players = query_as!(Player, "select * from players")
        .fetch_all(&manager.pool)
        .await?;

    // key: uuid, value: id
    let mut player_map = HashMap::new();
    for p in players {
        player_map.entry(p.uuid).or_insert(p.id);
    }

    let mut tx = manager
        .pool
        .begin()
        .await
        .context("Failed to initiate transaction")?;

    const CHUNK_SIZE: usize = 512;

    for player_stats in stats {
        let timestamp = player_stats.datetime;
        if let Some(id) = player_map.get(&player_stats.player_uuid) {
            for stats in player_stats.stats.chunks(CHUNK_SIZE) {
                let mut builder = crate::Builder::new(
                    "insert into stats (player_id, timestamp, category, name, value) ",
                );

                builder.push_values(stats, |mut b, stat| {
                    b.push_bind(id)
                        .push_bind(timestamp)
                        .push_bind(stat.category.to_string())
                        .push_bind(&stat.name)
                        .push_bind(stat.value as i32);
                });

                let result = builder.build().execute(&mut *tx).await;

                if let Err(e) = result {
                    error!("Failed inserting player stats in database: {}", e);
                } else {
                    info!("Inserted {} stats", CHUNK_SIZE);
                }
            }
        } else {
            error!(
                "Couldn't find player with UUID '{}' ",
                player_stats.player_uuid
            );
        }
    }

    let result = tx.commit().await;
    if let Err(e) = result {
        error!("Failed commiting transaction: {}", e);
    }

    Ok(())
}
