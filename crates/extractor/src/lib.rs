//! The `extractor` crate is responsible for extracting, grouping, filtering, ... stats
//!
//! It mainly is made to take `Vec<shared::PlayerSnapshot>`, and filter out stats needed for charts
//! For example:
//! - playtime for each player,
//! - total blocks mined for each player
//! - number of totems used for each player
//! - total playtime

// TODO: this crate might not be necessary, if i did it in database

use std::collections::HashMap;

use shared::{DataPoint, PlayerSeries, PlayerSnapshot};
use uuid::Uuid;

pub fn player_playtime(snapshots: &Vec<PlayerSnapshot>) -> Vec<PlayerSeries> {
    // key: player uuid, value: vector of datapoints
    let mut result: HashMap<Uuid, Vec<DataPoint>> = HashMap::new();

    for snapshot in snapshots {
        let Some(stat) = snapshot.stats.iter().find(|stat| stat.name == "play_time") else {
            continue;
        };

        result
            .entry(snapshot.player_uuid)
            .or_default()
            .push(DataPoint::new(snapshot.datetime, stat.value))
    }

    result
        .into_iter()
        .map(|(player_uuid, mut data_points)| {
            // sort by datetime
            data_points.sort_by_key(|point| point.x);
            PlayerSeries::new(player_uuid, data_points)
        })
        .collect::<Vec<_>>()
}
