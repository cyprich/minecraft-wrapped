use uuid::Uuid;

use crate::DataPoint;

/// PlayerSeries represents stats of certain category and value for one player  
///
/// For example:
/// - playtime of player
/// - totems used by player
///
/// This struct is mainly returned by functions in `extractor` crate
pub struct PlayerSeries {
    pub player_uuid: Uuid,
    pub data_points: Vec<DataPoint>,
}

impl PlayerSeries {
    pub fn new(player_uuid: Uuid, data_points: Vec<DataPoint>) -> Self {
        Self {
            player_uuid,
            data_points,
        }
    }
}
