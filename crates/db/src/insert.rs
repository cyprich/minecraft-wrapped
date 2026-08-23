use std::collections::HashMap;

use anyhow::Context;
use log::{error, info};
use shared::PlayerSnapshot;
use sqlx::{query_as, types::Uuid};

use crate::{Manager, models::PlayerRow};

/// Inserts player to database
pub async fn insert_player(
    manager: &Manager,
    uuid: &Uuid,
    name: Option<&str>,
    color_hex: Option<&str>,
) -> anyhow::Result<()> {
    let mut builder = crate::Builder::new("insert into players (uuid ");

    if name.is_some() {
        builder.push(", name");
    }
    if color_hex.is_some() {
        builder.push(", color_hex");
    }

    builder.push(" ) values ( ");
    builder.push_bind(uuid);

    if let Some(val) = name {
        builder.push(", ");
        builder.push_bind(val);
    }
    if let Some(val) = color_hex {
        builder.push(", ");
        builder.push_bind(val);
    }

    builder.push(" ) on conflict do nothing");

    builder.build().execute(&manager.pool).await?;

    Ok(())
}

/// Insert player snapshots
pub async fn insert_player_snapshots(
    manager: &Manager,
    snapshots: &[PlayerSnapshot],
) -> anyhow::Result<()> {
    let players = query_as!(PlayerRow, "select * from players")
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

    let mut inserted = 0;

    for player_stats in snapshots {
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

                builder.push(" on conflict do nothing");

                let result = builder.build().execute(&mut *tx).await;

                if let Err(e) = result {
                    error!("Failed inserting player stats in database: {}", e);
                } else {
                    inserted += CHUNK_SIZE;
                    info!("Inserted {} stats", inserted);
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
