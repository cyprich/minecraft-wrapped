use chrono::NaiveDateTime;

// TODO: make `y` generic
pub struct DataPoint {
    pub x: NaiveDateTime,
    pub y: u32,
}

impl DataPoint {
    pub fn new(x: NaiveDateTime, y: u32) -> Self {
        Self { x, y }
    }
}
