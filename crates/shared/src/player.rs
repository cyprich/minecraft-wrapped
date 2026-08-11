use anyhow::Context;
use uuid::Uuid;

pub struct Player {
    name: String,
    uuid: Uuid,
}

impl Player {
    pub fn new(name: &str, uuid: &str) -> anyhow::Result<Self> {
        let uuid = Uuid::parse_str(uuid).context(format!("Failed to parse '{}' to UUID", uuid))?;

        Ok(Self {
            name: name.to_string(),
            uuid,
        })
    }
}
