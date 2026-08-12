//! The `shared` crate contains mainly structs, which are used by multiple other libraries,
//! alongside some global constants

pub const DATETIME_FORMAT: &str = "%Y%m%d-%H%M%S";
pub const DATETIME_FORMAT_DISPLAY: &str = "YYYYMMDD-HHMMSS";

mod data_point;
mod player;
mod player_series;
mod player_snapshot;
mod player_snapshot_json;
mod stat_category;
mod stat_value;

pub use data_point::*;
pub use player::*;
pub use player_series::*;
pub use player_snapshot::*;
pub use player_snapshot_json::*;
pub use stat_category::*;
pub use stat_value::*;
