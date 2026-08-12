//! This is the command line interface for minecraft-wrapped

use anyhow::Context;
use log::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init simple logger
    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", log::LevelFilter::Off)
        .init()
        .context("Failed to initialize SimpleLogger")?;

    // select snapshots from db
    let db_manager = db::Manager::new().await?;
    let snapshots = db::select_player_snapshots(&db_manager).await?;
    info!("Loaded {} PlayerSnapshots", snapshots.len());

    // render playtime chart
    let playtime = extractor::player_playtime(&snapshots);
    if let Err(e) = charter::player_series("playtime.svg", "Playtime", &playtime) {
        error!("Error while rendering chart: {}", e);
    } else {
        info!("Chart succesfully rendered");
    }

    // render totems chart
    let totems = extractor::player_totems(&snapshots);
    if let Err(e) = charter::player_series("totems.svg", "Used Totems of Undying", &totems) {
        error!("Error while rendering chart: {}", e);
    } else {
        info!("Chart succesfully rendered");
    }

    Ok(())
}
