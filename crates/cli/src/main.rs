//! This is the command line interface for minecraft-wrapped

use std::{collections::HashSet, time::Instant};

use anyhow::Context;
use importer::Importer;
use log::{error, info};
use shared::PlayerStats;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init simple logger
    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", log::LevelFilter::Off)
        .init()
        .context("Failed to initialize SimpleLogger")?;

    // load everything from files
    let time = Instant::now();
    let raw = Importer::batch("data");
    let raw = match raw {
        Ok(val) => {
            info!(
                "Imported {} stats in {}s",
                val.len(),
                time.elapsed().as_secs_f32()
            );
            val
        }
        Err(e) => {
            error!("Error importing stats: {}", e);
            return Err(e);
        }
    };

    // convert all from json to `shared::PlayerStats`
    let time = Instant::now();
    let converted = raw
        .into_iter()
        .map(PlayerStats::from_raw)
        .filter_map(|s| s.ok())
        .collect::<Vec<_>>();
    info!(
        "Converted {} stats in {}s",
        converted.len(),
        time.elapsed().as_secs_f32()
    );

    let db_manager = db::Manager::new().await?;

    // TODO: temp insert players
    let players = converted
        .iter()
        .map(|s| s.player_uuid)
        .collect::<HashSet<_>>();

    for (id, uuid) in players.iter().enumerate() {
        let name = format!("player#{}", id);
        let result = db::insert_player(&db_manager, &name, uuid).await;

        if let Err(e) = result {
            error!("Error inserting player: {}", e);
        } else {
            info!("player#{}, uuid '{}' inserted", id, uuid);
        }
    }

    // TODO: temp insert stats
    let result = db::insert_player_stats(&db_manager, converted).await;

    if let Err(e) = result {
        error!("Failed inserting stats: {}", e);
    } else {
        info!("All stats inserted")
    }

    Ok(())
}
