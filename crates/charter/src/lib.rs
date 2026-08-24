//! The `charter` crate is responsible for generating/drawing charts

use std::path::Path;

use chrono::NaiveDateTime;
use plotters::prelude::*;
use shared::PlayerSeries;

use crate::models::Line;

mod models;

// TODO: temp
const PLAYER_COLORS: [&RGBColor; 6] = [&RED, &GREEN, &BLUE, &CYAN, &MAGENTA, &BLACK];

/// Render Line chart with multiple lines
/// **Expects `lines` to be sorted by datetime!**
fn render_lines(
    path: impl AsRef<Path>,
    caption: &str,
    lines: &[Line],
    y_desc: &str,
    y_unit: Option<&str>,
) -> anyhow::Result<()> {
    if lines.is_empty() {
        return Ok(());
    }

    // TODO: typst will need png/jpg
    let root = SVGBackend::new(path.as_ref(), (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    // create x_spec and y_spec
    let mut min_time = NaiveDateTime::MAX;
    let mut max_time = NaiveDateTime::MIN;
    let mut max_value: f64 = 0.0;
    for line in lines.iter() {
        for &(date, value) in &line.data {
            max_time = max_time.max(date);
            min_time = min_time.min(date);
            max_value = max_value.max(value);
        }
    }

    let x_spec = RangedDateTime::from(min_time..max_time);
    let y_spec = 0.0..max_value;

    // chart context
    let mut ctx = ChartBuilder::on(&root)
        // caption/title
        .caption(caption, ("Arial", 32))
        // y axis on the left
        .set_label_area_size(LabelAreaPosition::Left, 60)
        // x axis on the bottom
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .margin(4)
        .build_cartesian_2d(x_spec, y_spec)?;

    // descriptions
    let y_desc = match y_unit {
        Some(val) => &format!("{} [{}]", y_desc, val),
        None => y_desc,
    };

    // mesh
    ctx.configure_mesh()
        // TODO: Make these configurable
        .x_desc("Date")
        .y_desc(y_desc)
        .x_label_formatter(&|val| val.format("%d.%m.%Y").to_string())
        .draw()?;

    // render lines
    for line in lines {
        ctx.draw_series(LineSeries::new(
            line.data.iter().map(|(date, value)| (*date, *value)),
            line.style,
        ))?
        .label(&line.description)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line.style));
    }

    // legend
    ctx.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()?;

    // TODO: what does this do?
    root.present()?;

    Ok(())
}

pub fn player_series(
    path: impl AsRef<Path>,
    title: &str,
    data: &[PlayerSeries],
    y_desc: &str,
    y_unit: Option<&str>,
) -> anyhow::Result<()> {
    let mut i = 0;

    let lines = data
        .iter()
        .map(|data| {
            let description = format!("player {}", data.player_uuid);
            let data_points = data.data_points.iter().map(|p| (p.x, p.y as f64)).collect();

            let result = Line::new(
                data_points,
                &description,
                ShapeStyle {
                    color: (*PLAYER_COLORS[i % PLAYER_COLORS.len()]).into(),
                    filled: true,
                    stroke_width: 2,
                },
            );

            i += 1;
            result
        })
        .collect::<Vec<_>>();

    crate::render_lines(path, title, &lines, y_desc, y_unit)?;

    Ok(())
}
