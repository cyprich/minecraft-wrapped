#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub uuid: sqlx::types::Uuid,
}
