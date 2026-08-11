//! This module contains the PlayerStats struct  

use std::fmt::Debug;

use chrono::NaiveDateTime;

use crate::{RawPlayerStats, Stat, StatCategory};

/// RawPlayerStats contains:
/// - Player name/UUID
/// - datetime of stats
/// - stats - vector of `shared::Stat`, which is actual key-value-like struct
pub struct PlayerStats {
    pub player: String,
    pub stats: Vec<Stat>,
    pub datetime: NaiveDateTime,
}

impl PlayerStats {
    pub fn new(player: String, stats: Vec<Stat>, datetime: NaiveDateTime) -> Self {
        Self {
            player,
            stats,
            datetime,
        }
    }
}

impl From<RawPlayerStats> for PlayerStats {
    fn from(value: RawPlayerStats) -> Self {
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

        let mut stats: Vec<Stat> = Vec::new();
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

                        stats.push(Stat::new(category, name.clone(), value));
                    }
                }
            }
        }

        Self {
            player: value.player,
            stats,
            datetime: value.datetime,
        }
    }
}

impl Debug for PlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = format!("{} values", &self.stats.len());
        f.debug_struct("PlayerStats")
            .field("player", &self.player)
            .field("stats", &length)
            .field("datetime", &self.datetime)
            .finish()
    }
}
