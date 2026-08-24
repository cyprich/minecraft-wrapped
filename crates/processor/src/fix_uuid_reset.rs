use std::{collections::HashMap, time::Instant};

use log::trace;
use shared::PlayerSnapshot;

/// Fixes PlayerStats to be always rising, in case that Player UUID changed
///
/// Changes are made in-place
pub fn fix_uuid_reset(snapshots: &mut [PlayerSnapshot]) {
    // last values for each category and name
    // key: (stat category, stat name, player id), value: stat.value
    let mut last_map: HashMap<_, u32> = HashMap::new();

    // offsets for each category and name
    // key: (stat.Category, stat.name), value: stat.value
    let mut offset_map: HashMap<_, u32> = HashMap::new();

    let time = Instant::now();
    trace!("Starting to Fix UUID Reset...");

    for snapshot in snapshots.iter_mut() {
        for stat in &mut snapshot.stats {
            let key = (&stat.category, stat.name.as_str(), snapshot.player_id);

            let last = last_map.entry(key).or_insert(stat.value);

            if stat.value < *last {
                *offset_map.entry(key).or_default() += *last;
            }

            let offset = *offset_map.get(&key).unwrap_or(&0);

            *last = stat.value;
            stat.value += offset;
        }
    }

    trace!(
        "Finished Fixing UUID Reset in {}s",
        time.elapsed().as_secs_f32()
    );
}
