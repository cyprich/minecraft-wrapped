//! The `shared` crate contains mainly structs, which are used by multiple other libraries,
//! alongside some global constants

use log::error;
use uuid::Uuid;

pub const DATETIME_FORMAT: &str = "%Y%m%d-%H%M%S";
pub const DATETIME_FORMAT_DISPLAY: &str = "YYYYMMDD-HHMMSS";

mod data_point;
mod player;
mod player_series;
mod player_snapshot;
mod player_snapshot_json;
mod players;
mod stat_category;
mod stat_value;

pub use data_point::*;
pub use player::*;
pub use player_series::*;
pub use player_snapshot::*;
pub use player_snapshot_json::*;
pub use players::*;
pub use stat_category::*;
pub use stat_value::*;

pub fn str_to_uuid(value: &str) -> anyhow::Result<Uuid> {
    // check length of value
    let value = match value.len() {
        // 36 = expected length
        36 => value,
        // less than 36 = cannot be converted
        x if x < 36 => {
            return Err(anyhow::Error::msg(format!(
                "UUID too short, expected 36 characters: {}",
                value
            )));
        }
        // more than 36 = cut it to 36 and try it anyways
        _ => &value[..36],
    };

    match Uuid::parse_str(value) {
        Ok(val) => Ok(val),
        Err(e) => {
            error!("Unable to parse {} to UUID: {}", value, e);
            Err(e.into())
        }
    }
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let stripped = hex.strip_prefix("#").unwrap_or(hex);

    if stripped.len() != 6 {
        error!("Invalid HEX value: {}", hex);
        return None;
    };

    // parse from base 16
    let r = u8::from_str_radix(&stripped[0..2], 16).ok()?;
    let g = u8::from_str_radix(&stripped[2..4], 16).ok()?;
    let b = u8::from_str_radix(&stripped[4..6], 16).ok()?;

    Some((r, g, b))
}
