#[derive(Debug)]
pub struct PlayerRow {
    pub id: i32,
    pub uuid: sqlx::types::Uuid,
    pub name: Option<String>,
    pub color_hex: Option<String>,
}
