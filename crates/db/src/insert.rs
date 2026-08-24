use std::collections::HashMap;

use anyhow::Context;
use log::{error, trace};
use shared::PlayerSnapshot;
use sqlx::{query, query_as, types::Uuid};

use crate::{INSERT_CHUNK_SIZE, Manager, models::PlayerRow};

/// Inserts player UUID to database
pub async fn insert_player_uuid(manager: &Manager, uuid: &Uuid) -> anyhow::Result<()> {
    query!(
        "insert into player_uuids (uuid) values ($1) on conflict do nothing",
        uuid
    )
    .execute(&manager.pool)
    .await?;

    Ok(())
}

/// Insert player snapshots
pub async fn insert_player_snapshots(
    manager: &Manager,
    snapshots: &[PlayerSnapshot],
) -> anyhow::Result<()> {
    let mut tx = manager
        .pool
        .begin()
        .await
        .context("Failed to initiate transaction")?;

    let mut inserted = 0;

    for player_stats in snapshots {
        let timestamp = player_stats.datetime;
        for stats in player_stats.stats.chunks(INSERT_CHUNK_SIZE as usize) {
            let mut builder = crate::Builder::new(
                "insert into stats (player_id, timestamp, category, name, value) ",
            );

            builder.push_values(stats, |mut b, stat| {
                b.push_bind(player_stats.player_id)
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
                inserted += INSERT_CHUNK_SIZE;
                trace!("Inserted {} stats", inserted);
            }
        }
    }

    let result = tx.commit().await;
    if let Err(e) = result {
        error!("Failed commiting transaction: {}", e);
    }

    Ok(())
}
