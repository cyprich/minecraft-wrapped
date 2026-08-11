use std::fmt::Debug;

use chrono::NaiveDateTime;

pub const DATETIME_FORMAT: &str = "%Y%m%d-%H%M%S";
pub const DATETIME_FORMAT_DISPLAY: &str = "YYYYMMDD-HHMMSS";

pub struct RawPlayerStats {
    // TODO: name or UUID?
    player: String,
    stats: String,
    datetime: NaiveDateTime,
}

impl RawPlayerStats {
    pub fn new(player: String, stats: String, datetime: NaiveDateTime) -> Self {
        Self {
            player,
            stats,
            datetime,
        }
    }

    pub fn temp_simplify(&mut self) {
        self.stats = self.stats.replace("\n", "").replace(" ", "");
    }
}

impl Debug for RawPlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = format!("{} bytes", &self.stats.len());
        f.debug_struct("RawPlayerStats")
            .field("player", &self.player)
            .field("stats", &length)
            .field("datetime", &self.datetime)
            .finish()
    }
}
