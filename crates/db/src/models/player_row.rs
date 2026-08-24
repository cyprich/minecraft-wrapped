use shared::Player;

#[derive(Debug)]
pub struct PlayerRow {
    pub id: i32,
    pub name: Option<String>,
    pub color_hex: Option<String>,
}

impl PlayerRow {
    pub fn into_player(self) -> Player {
        Player {
            id: self.id,
            name: self.name,
            uuids: Vec::new(),
            color_hex: self.color_hex,
        }
    }
}
