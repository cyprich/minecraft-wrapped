use std::time::Instant;

use anyhow::Context;
use importer::Importer;
use log::{error, info};
use shared::PlayerStats;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::default()
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
    let converted = raw.into_iter().map(PlayerStats::from).collect::<Vec<_>>();
    info!(
        "Converted {} stats in {}s",
        converted.len(),
        time.elapsed().as_secs_f32()
    );

    Ok(())
}
