use chrono::NaiveDateTime;
use plotters::style::ShapeStyle;

pub struct Line {
    pub data: Vec<(NaiveDateTime, f64)>,
    pub description: String,
    pub style: ShapeStyle,
}

impl Line {
    pub fn new(
        data: Vec<(NaiveDateTime, f64)>,
        description: impl ToString,
        style: ShapeStyle,
    ) -> Self {
        Self {
            data,
            description: description.to_string(),
            style,
        }
    }
}
