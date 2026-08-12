//! The `extractor` crate is responsible for extracting, grouping, filtering, ... stats
//!
//! It mainly is made to take `Vec<shared::PlayerSnapshot>`, and filter out stats needed for charts
//! For example:
//! - playtime for each player,
//! - total blocks mined for each player
//! - number of totems used for each player
//! - total playtime

// TODO: this crate might not be necessary, if i did it in database

mod player_series;

pub use player_series::*;
