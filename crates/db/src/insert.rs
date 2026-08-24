use anyhow::Context;
use log::{error, trace};
use shared::PlayerSnapshot;
use sqlx::{query, query_scalar, types::Uuid};

use crate::{INSERT_CHUNK_SIZE, Manager};

/// Inserts Player to the database
/// Table `players`:
/// - id serial PK NN,
/// - name varchar(64),
/// - color_hex char(7)
pub async fn insert_player(
    manager: &Manager,
    name: Option<&str>,
    color_hex: Option<&str>,
) -> anyhow::Result<i32> {
    let id = query_scalar!(
        "insert into players (name, color_hex) values ($1, $2) returning id",
        name,
        color_hex
    )
    .fetch_one(&manager.pool)
    .await?;

    Ok(id)
}

/// Inserts player UUID to database
/// Table `player_uuids`:
/// - uuid uuid PK NN,
/// - player_id integer references players(id)
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
                inserted += stats.len();
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
