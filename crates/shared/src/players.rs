use std::collections::HashMap;

use uuid::Uuid;

use crate::Player;

pub struct Players {
    vec: Vec<Player>,
    uuid_to_id_map: HashMap<Uuid, i32>,
}

impl Players {
    pub fn new(players: &[Player]) -> Self {
        let mut uuid_to_id_map = HashMap::new();

        for p in players {
            for u in &p.uuids {
                uuid_to_id_map.entry(*u).insert_entry(p.id);
            }
        }

        Self {
            vec: players.into(),
            uuid_to_id_map,
        }
    }

    // pub fn iter(&self) -> std::slice::Iter<'_, Player> {
    //     self.vec.iter()
    // }

    pub fn iter(&self) -> impl Iterator<Item = &Player> {
        self.vec.iter()
    }

    pub fn get_player_by_id(&self, id: i32) -> Option<&Player> {
        self.vec.iter().find(|p| p.id == id)
    }

    pub fn get_player_by_uuid(&self, uuid: &Uuid) -> Option<&Player> {
        self.vec.iter().find(|p| p.uuids.contains(uuid))
    }

    pub fn get_id_by_uuid(&self, uuid: &Uuid) -> Option<i32> {
        self.uuid_to_id_map.get(uuid).copied()
    }
}

impl From<Vec<Player>> for Players {
    fn from(value: Vec<Player>) -> Self {
        Self::new(&value)
    }
}
