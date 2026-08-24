use crate::DataPoint;

/// PlayerSeries represents stats of certain category and value for one player  
///
/// For example:
/// - playtime of player
/// - totems used by player
///
/// This struct is mainly returned by functions in `extractor` crate
pub struct PlayerSeries {
    pub player_display_name: String,
    pub player_color: Option<String>,
    pub data_points: Vec<DataPoint>,
}

impl PlayerSeries {
    pub fn new(
        player_display_name: &str,
        player_color: Option<String>,
        data_points: Vec<DataPoint>,
    ) -> Self {
        Self {
            player_display_name: player_display_name.to_string(),
            player_color,
            data_points,
        }
    }
}
