//! The `charter` crate is responsible for generating/drawing charts

use std::{ops::Range, path::Path};

use chrono::NaiveDateTime;
use plotters::prelude::*;
use shared::PlayerSeries;

use crate::models::Line;

mod models;

pub fn sample_chart() -> anyhow::Result<()> {
    // let root = BitMapBackend::new("sample_chart.png", (1920, 1080)).into_drawing_area();
    let root = SVGBackend::new("sample_chart.svg", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    const X_SPEC: Range<f64> = 0.0..10.0;
    const Y_SPEC: Range<f64> = -1.1..1.1;

    // chart context
    let mut ctx = ChartBuilder::on(&root)
        // caption/title
        .caption("This is sample chart", ("Arial", 32))
        // y axis on the left
        .set_label_area_size(LabelAreaPosition::Left, 60)
        // x axis on the bottom
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .build_cartesian_2d(X_SPEC, Y_SPEC)?;

    ctx.configure_mesh().draw()?;

    // draw stuff
    ctx.draw_series(LineSeries::new(
        (0..1000).map(|i| {
            let i = (i as f64) / 100.0;
            (i, i.sin())
        }),
        &RED,
    ))?;

    Ok(())
}

/// Render Line chart with multiple lines
/// **Expects `lines` to be sorted by datetime!**
fn render_lines(
    path: impl AsRef<Path>,
    caption: &str,
    // lines: Vec<(Vec<(f64, f64)>, String, impl Into<ShapeStyle>)>,
    lines: &[Line],
) -> anyhow::Result<()> {
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
        .build_cartesian_2d(x_spec, y_spec)?;

    ctx.configure_mesh().draw()?;

    // for (datapoints, description, style) in lines {
    for line in lines {
        ctx.draw_series(LineSeries::new(
            line.data.iter().map(|(date, value)| (*date, *value)),
            line.style,
        ))?
        .label(&line.description)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line.style));
    }

    // TODO: what does this do?
    root.present()?;

    Ok(())
}

pub fn player_playtime(data: &[PlayerSeries]) -> anyhow::Result<()> {
    let colors = [&RED, &GREEN, &BLUE, &CYAN, &MAGENTA, &YELLOW, &BLACK];
    let mut i = 0;

    let lines = data
        .iter()
        .map(|data| {
            let description = format!("player {}", data.player_uuid);
            let data_points = data.data_points.iter().map(|p| (p.x, p.y as f64)).collect();

            let result = Line::new(data_points, &description, colors[i % colors.len()].into());
            i += 1;
            result
        })
        .collect::<Vec<_>>();

    crate::render_lines("playtime.svg", "Playtime for each player", &lines)?;

    Ok(())
}
