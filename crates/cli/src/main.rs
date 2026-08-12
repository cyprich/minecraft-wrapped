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

    let playtime = extractor::player_playtime(&snapshots);

    let result = charter::player_playtime(&playtime);
    match result {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering charts: {}", e),
    }

    Ok(())
}
