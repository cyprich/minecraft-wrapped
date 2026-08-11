//! This module contains the PlayerStats struct  

use std::fmt::Display;

/// Stat contains one statistic, which is:
/// - category: `mined`
/// - name: `cobblestone`
/// - value: 123
#[derive(Debug)]
pub struct Stat {
    pub category: StatCategory,
    pub name: String,
    pub value: u32,
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
    Picked,
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
            "pickedup" => Some(Self::Picked),
            "used" => Some(Self::Used),
            _ => None,
        }
    }
}

impl Display for StatCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            StatCategory::Crafted => "crafted",
            StatCategory::Custom => "custom",
            StatCategory::Dropped => "dropped",
            StatCategory::Killed => "killed",
            StatCategory::Mined => "mined",
            StatCategory::Picked => "picked",
            StatCategory::Used => "used",
        };

        write!(f, "{}", val)
    }
}
