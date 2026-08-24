use std::{collections::HashSet, fmt::Display};

use log::warn;
use uuid::Uuid;

use crate::str_to_uuid;

/// Represents a Player
/// Each player is identified by his name  
/// Each player can have multiple UUID's, because of  
#[derive(Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub name: Option<String>,
    pub uuids: Vec<Uuid>,
    pub color_hex: Option<String>,
}

impl Player {
    /// Creates
    pub fn new(
        id: i32,
        name: Option<&str>,
        uuids: &[&str],
        color_hex: Option<&str>,
    ) -> anyhow::Result<Self> {
        // normalize format of `color_hex`, convert to String
        let color_hex = if let Some(color_hex) = color_hex {
            match (color_hex.len(), color_hex.starts_with("#")) {
                (7, true) => Some(color_hex.to_string()),
                (6, false) => Some(format!("#{}", color_hex)),
                _ => {
                    warn!("Invalid format of `hex_string`: {}", color_hex);
                    None
                }
            }
        } else {
            None
        };

        // convert &[&str] to HashSet<Uuid>
        let uuids = uuids
            .iter()
            .filter_map(|uuid| match str_to_uuid(uuid) {
                Ok(val) => Some(val),
                Err(e) => {
                    warn!("Invalid UUID format: {}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(Self {
            id,
            name: name.map(String::from),
            uuids,
            color_hex,
        })
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "#{}", self.id),
        }
    }
}
