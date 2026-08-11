use anyhow::Context;
use importer::Importer;
use log::{error, info};

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::default()
        .init()
        .context("Failed to initialize SimpleLogger")?;

    let import = Importer::batch("data");
    match &import {
        Ok(val) => {
            info!("Imported {} stats", val.len());
        }
        Err(e) => {
            error!("Error importing stats: {}", e);
        }
    }

    Ok(())
}
