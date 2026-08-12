use std::fmt::Display;

/// Enum of categories of stats in Minecraft
#[derive(Debug, PartialEq)]
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
