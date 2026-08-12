//! The `importer` crate responsible for importing stats from files
//!
//! Stats are just read as `shared::RawJsonStats`,
//! meaning the stats themselves are not parsed into structs,

use std::{fs, path::Path};

use anyhow::Context;
use log::{error, warn};
use shared::PlayerSnapshotJson;

/// Reads raw stats from files
///
/// `batch` reads stats of multiple players from *multiple days*
/// `simple` read stats of multiple players from *one day*
pub struct Importer {}

impl Importer {
    /// Reads stats of multiple players from multiple days
    ///
    /// Expected structure of input folder:
    /// ```
    /// folder  
    /// ├── 20260101-050000
    /// │   ├── uuid1.json
    /// │   ├── uuid2.json
    /// │   └── uuid3.json
    /// ├── 20260102-050000
    /// │   ├── uuid1.json
    /// │   ├── uuid2.json
    /// │   └── uuid3.json
    /// └── ...
    /// ```
    pub fn batch(folder: impl AsRef<Path>) -> anyhow::Result<Vec<PlayerSnapshotJson>> {
        let folder = folder.as_ref();

        if !fs::exists(folder).unwrap_or(false) {
            return Err(anyhow::Error::msg(format!(
                "'{}' does not exist",
                folder.display()
            )));
        };

        if !folder.is_dir() {
            return Err(anyhow::Error::msg(format!(
                "'{}' is not a folder",
                folder.display()
            )));
        }

        let result = fs::read_dir(folder)
            .context(format!("Failed to read directory '{}'", folder.display()))?
            .filter_map(|f| match f {
                // until iterator has next value i guess
                Ok(val) => Some(val),
                Err(e) => {
                    warn!("{}", e);
                    None
                }
            })
            .filter_map(|f| {
                // read contents
                let result = Self::simple(f.path());
                match result {
                    Ok(val) => Some(val),
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        Ok(result)
    }

    /// Reads stats of multiple players from one day
    ///
    /// Expected structure of input folder:
    /// ```
    /// 20260101-050000
    /// ├── uuid1.json
    /// ├── uuid2.json
    /// └── uuid3.json
    /// ```
    pub fn simple(folder: impl AsRef<Path>) -> anyhow::Result<Vec<PlayerSnapshotJson>> {
        let folder = folder.as_ref();

        // check if folder exists
        if !fs::exists(folder).unwrap_or(false) {
            return Err(anyhow::Error::msg(format!(
                "'{}' does not exist",
                folder.display()
            )));
        };

        // check if folder is actually an folder
        if !folder.is_dir() {
            return Err(anyhow::Error::msg(format!(
                "'{}' is not a folder",
                folder.to_string_lossy()
            )));
        }

        // extract datetime from folder name
        let datetime = folder.to_string_lossy();
        let datetime = datetime.rsplit("/").next().context(format!(
            "Failed to extract subfolder name from '{}'",
            folder.display()
        ))?;
        let datetime = chrono::NaiveDateTime::parse_from_str(datetime, shared::DATETIME_FORMAT)
            .context(format!(
                "Failed to parse '{}' to '{}' format ({})",
                datetime,
                shared::DATETIME_FORMAT,
                shared::DATETIME_FORMAT_DISPLAY
            ))?;

        // read stats and player names from json files
        let result = fs::read_dir(folder)
            .context(format!(
                "Failed to read the content of '{}'",
                folder.display()
            ))?
            // until the iterator has next value
            .filter_map(|f| f.ok())
            // extract player name and stats
            .filter_map(|f| {
                let path = f.path();

                // extract player name
                let player_name = path.file_stem().and_then(|s| s.to_str());
                let player_name = if let Some(val) = player_name {
                    val.to_string()
                } else {
                    error!(
                        "Failed to extract player name from '{}'",
                        f.path().display()
                    );
                    return None;
                };

                // extract stats (content of json file)
                let stats = fs::read_to_string(path);
                let stats = match stats {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Failed reading content of '{}': {}", f.path().display(), e);
                        return None;
                    }
                };
                // TODO: this might cause problems later (serde), idk
                let stats = stats.replace("\n", "").replace(" ", "");

                Some((stats, player_name))
            })
            // construct RawPlayerStats
            .map(|(stats, player_name)| PlayerSnapshotJson::new(player_name, stats, datetime))
            .collect::<Vec<_>>();

        Ok(result)
    }
}
