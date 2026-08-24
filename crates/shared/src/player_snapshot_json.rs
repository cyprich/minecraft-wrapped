//! This module contains the RawPlayerStats struct  

use chrono::NaiveDateTime;
use std::fmt::Debug;

/// PlayerSnapshotJson represents all stats (in JSON String) belonging to one player at one point in time:
///
/// The structure is
/// - player UUID
/// - datetime
/// - raw JSON stats in String, which needs to be converted to `shared::StatValue`
pub struct PlayerSnapshotJson {
    pub player_uuid: String,
    pub json: String,
    pub datetime: NaiveDateTime,
}

impl PlayerSnapshotJson {
    pub fn new(player_uuid: String, json: String, datetime: NaiveDateTime) -> Self {
        Self {
            player_uuid,
            json,
            datetime,
        }
    }
}

impl Debug for PlayerSnapshotJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = format!("{} bytes", self.json.len());
        f.debug_struct("RawPlayerStats")
            .field("player", &self.player_uuid)
            .field("stats", &length)
            .field("datetime", &self.datetime)
            .finish()
    }
}
