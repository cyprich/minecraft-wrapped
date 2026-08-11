//! This module contains the RawPlayerStats struct  

use chrono::NaiveDateTime;
use std::fmt::Debug;

/// RawPlayerStats contains:
/// - Player name/UUID
/// - datetime of stats
/// - raw stats - content of JSON file stored in String (no converstion to struct)
pub struct RawPlayerStats {
    // TODO: name or UUID?
    pub player: String,
    pub json: String,
    pub datetime: NaiveDateTime,
}

impl RawPlayerStats {
    pub fn new(player: String, json: String, datetime: NaiveDateTime) -> Self {
        Self {
            player,
            json,
            datetime,
        }
    }
}

impl Debug for RawPlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = format!("{} bytes", &self.json.len());
        f.debug_struct("RawPlayerStats")
            .field("player", &self.player)
            .field("stats", &length)
            .field("datetime", &self.datetime)
            .finish()
    }
}
