//! This module contains the PlayerStats struct  

use std::fmt::Debug;

use anyhow::Context;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{PlayerSnapshotJson, StatCategory, StatValue};

/// PlayerSnapshot represents all stats belonging to one player at one point in time:
///
/// The structure is
/// - player UUID
/// - datetime
/// - stats - vector of `shared::StatValue`, which is actual key-value-like struct
pub struct PlayerSnapshot {
    pub player_uuid: Uuid,
    pub stats: Vec<StatValue>,
    pub datetime: NaiveDateTime,
}

impl PlayerSnapshot {
    pub fn new(
        player_uuid: String,
        stats: Vec<StatValue>,
        datetime: NaiveDateTime,
    ) -> anyhow::Result<Self> {
        let player_uuid = Uuid::parse_str(&player_uuid)
            .context(format!("Failed to parse '{}' to uuid", player_uuid))?;

        Ok(Self {
            player_uuid,
            stats,
            datetime,
        })
    }

    pub fn from_uuid(player_uuid: Uuid, stats: Vec<StatValue>, datetime: NaiveDateTime) -> Self {
        Self {
            player_uuid,
            stats,
            datetime,
        }
    }
}

impl PlayerSnapshot {
    /// Tries to convert `PlayerSnapshotJson` to `PlayerSnapshot`
    ///
    /// Uses `serde_json` library to read the JSON string,
    /// and tries to convert it to `shared::StatValue` struct
    pub fn from_raw(value: PlayerSnapshotJson) -> anyhow::Result<Self> {
        // the json from minecraft looks something like this:
        //
        //  {
        //      "stats": {
        //          "minecraft:mined": {
        //              "minecraft:stone": 123,
        //              "minecraft:dirt": 456
        //          },
        //          "minecraft:killed": {
        //              "minecraft:zombie": 10,
        //              "minecraft:spider": 20
        //          },
        //          "minecraft:custom": {
        //              "minecraft:play_time": 123456,
        //              "minecraft:jump": 123456,
        //              "minecraft:walk_one_cm": 123456
        //          }
        //      }
        //      "DataVersion": 1234
        //  }

        let mut stats: Vec<StatValue> = Vec::new();
        let raw: serde_json::Value = serde_json::from_str(&value.json).unwrap();

        // get only the `stats` part
        if let Some(categories) = raw.get("stats").and_then(|c| c.as_object()) {
            for category in categories {
                // category_name would be `minecraft:mined`, `minecraft:killed`, ...
                let (category_name, category_values) = category;
                if let Some(category_values) = category_values.as_object() {
                    for (stat_name, stat_value) in category_values {
                        // category_name: `minecraft:killled`
                        // stat_name: `minecraft:zombie`
                        // stat_value: `123`

                        let value = match stat_value.as_i64() {
                            Some(val) => val as u32,
                            None => continue,
                        };
                        let category = match StatCategory::try_from_str(category_name) {
                            Some(val) => val,
                            None => continue,
                        };

                        let name = stat_name.replace("minecraft:", "");

                        stats.push(StatValue::new(category, name.clone(), value));
                    }
                }
            }
        }

        Self::new(value.player_uuid, stats, value.datetime)
    }
}

impl Debug for PlayerSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = format!("{} values", &self.stats.len());
        f.debug_struct("PlayerStats")
            .field("player_name", &self.player_uuid)
            .field("stats", &length)
            .field("datetime", &self.datetime)
            .finish()
    }
}
