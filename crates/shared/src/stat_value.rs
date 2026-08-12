use crate::StatCategory;

/// StatValue represents actual Minecraft statistic
///
/// The structure is
/// - category: `shared::StatCategory` - e.g. `mined`, `killed`, `dropped`
/// - name: `String` - e.g. `cobblestone`, `zombie`, `oak_log`
/// - value: u32 - e.g. 123
#[derive(Debug)]
pub struct StatValue {
    pub category: StatCategory,
    pub name: String,
    pub value: u32,
}

impl StatValue {
    pub fn new(category: StatCategory, name: String, value: u32) -> Self {
        Self {
            category,
            name,
            value,
        }
    }
}
