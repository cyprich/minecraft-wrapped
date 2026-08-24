use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct StatRow {
    pub player_id: i32,
    pub timestamp: NaiveDateTime,
    pub category: String,
    pub name: String,
    pub value: i32,
}
