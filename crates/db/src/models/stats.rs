use sqlx::types::chrono::NaiveDateTime;

pub struct Stats {
    pub player_id: i32,
    pub timestamp: NaiveDateTime,
    pub category: String,
    pub name: String,
    pub value: i32,
}
