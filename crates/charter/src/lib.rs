use std::{ops::Range, path::Path};

use plotters::{prelude::*, style};

pub struct Charter {}

impl Charter {
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
    ///
    /// You can image the `lines` parameter as this:
    /// ```
    /// Lines {
    ///     Vec<Line {
    ///         Vec<Data {
    ///             x: f64,
    ///             y: f64
    ///         }>,
    ///         style: impl Into<plotters::ShapeStyle>,  # color, stroke width, ...
    ///         description: &str
    ///     }>
    /// }
    /// ```
    ///
    /// ...basically vector of lines;
    /// `Line` consists of `description`, `style` (color) and a bunch of `x, y` pairs
    pub fn render_lines(
        path: impl AsRef<Path>,
        caption: &str,
        // lines: Vec<(Vec<(f64, f64)>, String, impl Into<ShapeStyle>)>,
        lines: Vec<(Vec<(f64, f64)>, String)>,
    ) -> anyhow::Result<()> {
        let root = SVGBackend::new(path.as_ref(), (1920, 1080)).into_drawing_area();
        root.fill(&WHITE)?;

        // create x_spec and y_spec
        let (x_min, x_max, y_min, y_max) = lines.iter().flat_map(|l| &l.0).fold(
            // we are starting with these accumulator values, which will be updated
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            // `|(current_accumulator_values), (current_iterator_values)|`
            // if x_min < x, update it,
            // if x_max > x, update it, ...
            |(x_min, x_max, y_min, y_max), &(x, y)| {
                (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
            },
        );

        let x_spec = x_min..x_max;
        let y_spec = y_min..y_max;

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
        for (datapoints, description) in lines {
            let color = &BLACK;
            ctx.draw_series(LineSeries::new(datapoints, color))?
                .label(description)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        // TODO: what does this do?
        root.present()?;

        Ok(())
    }
}
