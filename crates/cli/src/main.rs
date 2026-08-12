//! This is the command line interface for minecraft-wrapped

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use anyhow::Context;
use charter::Charter;
use importer::Importer;
use log::{error, info, trace};
use shared::{PlayerStats, StatCategory};

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

    info!("Loaded {} PlayerStats", stats.len());
    // info!("PlayerStats: {:?}", stats);

    // sample chart
    let result = Charter::sample_chart();
    match result {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering charts: {}", e),
    }

    // TODO: fuck this tho, i need to use just db::models::Stat

    // group stats by player
    // key: player_uuid, value: (datetime, Vec<Stat>)
    let mut stats_map = HashMap::new();
    for stat in stats.iter().filter(|player_stats| {
        player_stats.stats.iter().filter(|stat| {
            matches!(stat.category, StatCategory::Custom) && stat.name == "play_time"
        })
    }) {
        stats_map
            .entry(stat.player_uuid)
            .or_insert_with(|| (stat.datetime, &stat.stats));
    }

    // playtime chart
    // let playtimes = stats
    //     .iter()
    //     .filter_map(|player_stat| {
    //         let caption = player_stat.player_uuid.to_string();
    //         let datapoints = player_stat
    //             .stats
    //             .iter()
    //             .enumerate()
    //             .filter_map(|(n, stat)| {
    //                 if matches!(stat.category, StatCategory::Custom) && stat.name == "play_time" {
    //                     // trace!("Stat: {}::{}::{}", caption, n, stat.value);
    //                     Some((n as f64, stat.value as f64))
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect::<Vec<_>>();
    //
    //         if !datapoints.is_empty() {
    //             // info!("Got {} datapoints", datapoints.len());
    //             Some((datapoints, caption))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // playtimes
    let playtimes = stats_map
        .iter()
        .filter_map(|(k, v)| {
            let datapoints = v
                .1
                .iter()
                .enumerate()
                .filter_map(|(n, stat)| {
                    if matches!(stat.category, StatCategory::Custom) && stat.name == "play_time" {
                        Some((n as f64, stat.value as f64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let caption = k.to_string();

            if !datapoints.is_empty() {
                Some((datapoints, caption))
            } else {
                None
            }

            // let values = v.1.iter().filter(|stat| {
            //     matches!(stat.category, StatCategory::Custom) && stat.name == "play_time"
            // }).collect();
        })
        .collect::<Vec<_>>();

    let result = Charter::render_lines("playtimes.svg", "Playtimes", playtimes);
    match result {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering charts: {}", e),
    }

    Ok(())
}
