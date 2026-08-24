use uuid::Uuid;

use crate::str_to_uuid;

/// Represents a Player
/// Each player is identified by his name  
/// Each player can have multiple UUID's, because of  
pub struct Player {
    name: String,
    uuids: Vec<Uuid>,
    color_hex: String,
}

impl Player {
    /// Creates
    pub fn new(name: &str, uuids: &[&str], color_hex: &str) -> anyhow::Result<Self> {
        // check format of `color_hex`, convert to String
        let color_hex = match (color_hex.len(), color_hex.starts_with("#")) {
            (7, true) => color_hex.to_string(),
            (6, false) => format!("#{}", color_hex),
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Invalid format of `hex_string`: {}",
                    color_hex
                )));
            }
        };

        let uuids = uuids
            .iter()
            .filter_map(|uuid| str_to_uuid(uuid).ok())
            .collect::<Vec<_>>();

        Ok(Self {
            name: name.to_string(),
            uuids,
            color_hex,
        })
    }
}
