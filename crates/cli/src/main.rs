//! This is the command line interface for minecraft-wrapped

use std::{collections::HashSet, time::Instant};

use anyhow::Context;
use charter::Charter;
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

    // temp disable this
    if false {
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

            match result {
                Ok(_) => info!("player#{}, uuid '{}' inserted", id, uuid),
                Err(e) => error!("Error inserting player: {}", e),
            }
        }

        // TODO: temp insert stats
        let result = db::insert_player_stats(&db_manager, converted).await;

        match result {
            Ok(_) => info!("All stats inserted"),
            Err(e) => error!("Failed inserting stats: {}", e),
        }
    }

    // stats
    let db_manager = db::Manager::new().await?;
    let stats = match db::select_player_stats(&db_manager).await {
        Ok(val) => val,
        Err(e) => {
            error!("Error while getting PlayerStats: {}", e);
            return Err(e);
        }
    };

    info!("Loaded {} PlayerStats: {:?}", stats.len(), stats);

    // charts
    let result = Charter::sample_chart();
    match result {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering charts: {}", e),
    }

    Ok(())
}
