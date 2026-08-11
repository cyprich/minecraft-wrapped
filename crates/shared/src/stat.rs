//! This module contains the PlayerStats struct  

/// Stat contains one statistic, which is:
/// - category: `mined`
/// - name: `cobblestone`
/// - value: 123
#[derive(Debug)]
pub struct Stat {
    category: StatCategory,
    name: String,
    value: u32,
}

impl Stat {
    pub fn new(category: StatCategory, name: String, value: u32) -> Self {
        Self {
            category,
            name,
            value,
        }
    }
}

/// Categories of stats in Minecraft
#[derive(Debug)]
pub enum StatCategory {
    Crafted,
    Custom,
    Dropped,
    Killed,
    Mined,
    PickedUp,
    Used,
}

impl StatCategory {
    pub fn try_from_str(category: &str) -> Option<Self> {
        let category = match category.starts_with("minecraft:") {
            true => category.replace("minecraft:", ""),
            false => category.to_string(),
        };

        match category.to_lowercase().as_str() {
            "crafted" => Some(Self::Crafted),
            "custom" => Some(Self::Custom),
            "dropped" => Some(Self::Dropped),
            "killed" => Some(Self::Killed),
            "mined" => Some(Self::Mined),
            "pickedup" => Some(Self::PickedUp),
            "used" => Some(Self::Used),
            _ => None,
        }
    }
}
