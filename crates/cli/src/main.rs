//! This is the command line interface for minecraft-wrapped

use anyhow::Context;
use log::{error, info};
use shared::PlayerSnapshot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init simple logger
    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", log::LevelFilter::Off)
        .init()
        .context("Failed to initialize SimpleLogger")?;

    // initialize db manager
    let db_manager = db::Manager::new().await?;

    // import player UUID's from files, insert them to db
    let import_players = true;
    if import_players {
        let uuids = importer::batch_players("data")?;
        for uuid in uuids {
            db::insert_player(&db_manager, &uuid, None, None).await?;
        }
    }

    // import snapshots from files, insert them to db
    let import_snapshots = true;
    if import_snapshots {
        let raw = importer::batch_snapshots("data")?;
        let snapshots = raw
            .into_iter()
            .filter_map(|s| PlayerSnapshot::from_raw(s).ok())
            .collect::<Vec<_>>();
        db::insert_player_snapshots(&db_manager, &snapshots).await?;
    }

    // select snapshots from db
    let snapshots = db::select_player_snapshots(&db_manager).await?;
    info!("Loaded {} PlayerSnapshots", snapshots.len());

    // render playtime chart
    let playtime = extractor::player_playtime(&snapshots);
    match charter::player_series("playtime.svg", "Playtime", &playtime) {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering chart: {}", e),
    }

    // render totems chart
    let totems = extractor::player_totems(&snapshots);
    match charter::player_series("totems.svg", "Used Totems of Undying", &totems) {
        Ok(_) => info!("Chart succesfully rendered"),
        Err(e) => error!("Error while rendering chart: {}", e),
    }

    Ok(())
}
