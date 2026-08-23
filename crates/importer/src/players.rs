use std::{collections::HashSet, fs, path::Path};

use anyhow::Context;
use log::{error, warn};
use shared::str_to_uuid;
use uuid::Uuid;

// TODO: this is pretty significant duplicate with `crate::snapshots`

/// Reads player UUID's players from *multiple days*
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
pub fn batch_players(folder: impl AsRef<Path>) -> anyhow::Result<HashSet<Uuid>> {
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
            let result = crate::simple_players(f.path());
            match result {
                Ok(val) => Some(val),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        })
        .flatten()
        .collect::<HashSet<_>>();

    Ok(result)
}

/// Reads player UUID's from *one day*
///
/// Expected structure of input folder:
/// ```
/// 20260101-050000
/// ├── uuid1.json
/// ├── uuid2.json
/// └── uuid3.json
/// ```
pub fn simple_players(folder: impl AsRef<Path>) -> anyhow::Result<HashSet<Uuid>> {
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

    let result = fs::read_dir(folder)
        .context(format!(
            "Failed to read the content of '{}'",
            folder.display()
        ))?
        // until the iterator has next value
        .filter_map(|f| f.ok())
        // extract UUID's
        .filter_map(|f| {
            let path = f.path();
            let value = path.file_stem().and_then(|f| f.to_str());
            match value {
                Some(value) => str_to_uuid(value).ok(),
                None => None,
            }
        })
        .collect::<HashSet<_>>();

    Ok(result)
}
