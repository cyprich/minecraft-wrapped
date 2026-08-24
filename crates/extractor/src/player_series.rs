use shared::{DataPoint, PlayerSeries, PlayerSnapshot, Players, StatCategory};
use std::collections::HashMap;

fn filter_snapshots(
    snapshots: &[PlayerSnapshot],
    players: &Players,
    stat_name: Option<&str>,
    stat_category: Option<StatCategory>,
) -> Vec<PlayerSeries> {
    // key: player id, value: vector of datapoints
    let mut result: HashMap<i32, Vec<DataPoint>> = HashMap::new();

    for snapshot in snapshots {
        // check if stat name and category matches parameters
        let Some(stat) = snapshot.stats.iter().find(|stat| {
            let name_matches = match stat_name {
                Some(val) => stat.name == val,
                // if it was not specified, we want to ignore it, so it always matches
                None => true,
            };

            let category_matches = match &stat_category {
                Some(val) => &stat.category == val,
                None => true,
            };

            name_matches && category_matches
        }) else {
            continue;
        };

        // collecto into hashmap, gruped by player uuid
        result
            .entry(snapshot.player_id)
            .or_default()
            .push(DataPoint::new(snapshot.datetime, stat.value))
    }

    // make hashmap into vector that is returned
    result
        .into_iter()
        .map(|(player_id, mut data_points)| {
            // sort by datetime
            data_points.sort_by_key(|point| point.x);
            let player = players.get_player_by_id(player_id);
            dbg!(&player);
            let name = match &player {
                Some(p) => p.to_string(),
                None => format!("#{}", player_id),
            };
            let color = match &player {
                Some(p) if let Some(c) = &p.color_hex => Some(c.clone()),
                _ => None,
            };
            PlayerSeries::new(&name, color, data_points)
        })
        .collect::<Vec<_>>()
}

pub fn player_playtime(snapshots: &[PlayerSnapshot], players: &Players) -> Vec<PlayerSeries> {
    let result = filter_snapshots(snapshots, players, Some("play_time"), None);

    // convert minecraft ticks to hours
    result
        .into_iter()
        .map(|mut series| {
            for point in &mut series.data_points {
                point.y /= 20 * 60 * 60;
            }

            series
        })
        .collect()
}

pub fn player_totems(snapshots: &[PlayerSnapshot], players: &Players) -> Vec<PlayerSeries> {
    filter_snapshots(
        snapshots,
        players,
        Some("totem_of_undying"),
        Some(StatCategory::Used),
    )
}
