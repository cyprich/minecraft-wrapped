//! The `processor` crate is responsible for extracting, grouping, filtering, ... stats
//!
//! It mainly is made to take `&[shared::PlayerSnapshot]`, and filter out stats needed for charts
//! For example:
//! - playtime for each player,
//! - total blocks mined for each player
//! - number of totems used for each player
//! - total playtime

// TODO: this crate might not be necessary, if i did it in database

mod fix_uuid_reset;
mod player_series;

pub use fix_uuid_reset::*;
pub use player_series::*;
