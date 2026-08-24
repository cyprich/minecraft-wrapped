use sqlx::types::Uuid;

pub struct PlayerUuidRow {
    pub uuid: Uuid,
    pub player_id: Option<i32>,
}
