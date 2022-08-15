use crate::plot::colors::*;
use plotters::chart::SeriesAnno;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;
use rand_distr::DistIter;

pub type Chart<'a, 'b> = ChartContext<
    'a,
    BitMapBackend<'b>,
    Cartesian2d<RangedCoordf64, RangedCoordf64>,
>;

// This is a macro to avoid lifetime issues from CT due to root
macro_rules! basic_chart {
    ($root:expr) => {
        ChartBuilder::on($root)
            .margin_left(10_i32)
            .margin_right(30_i32)
            .margin_top(10_i32)
            .margin_bottom(10_i32)
            .x_label_area_size(40_i32)
            .y_label_area_size(80_i32)
    };
}

// macro used to avoid lifetime errors
macro_rules! chart_with_mesh {
    ($root: expr, $y_range: expr) => {{
        let mut chart = basic_chart!($root)
            .margin_top(30_i32)
            .build_cartesian_2d(-10.0..110.0_f64, $y_range)?;

        chart
            .configure_mesh()
            .axis_desc_style(("sans-serif", 20_i32).into_text_style($root))
            .light_line_style(&WHITE)
            .draw()?;
        chart
    }};
}

macro_rules! add_legend {
    ($chart: expr) => {{
        $chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(WHITE.filled())
            .border_style(&BLACK.mix(0.5))
            .legend_area_size(22_i32)
            .label_font(("sans-serif", 20_i32))
            .draw()?;
    }};
}

pub fn plot_platform_bounds(
    chart: &Chart,
    root: &DrawingArea<BitMapBackend, Shift>,
    modifier: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let drawing_area = chart.plotting_area();
    let mapped = drawing_area.map_coordinate(&(0.0, 0.0));
    let p: PathElement<(i32, i32)> = PathElement::new(
        [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
        black_stroke(),
    );
    root.draw(&p)?;

    let mapped = drawing_area.map_coordinate(&(100.0, 0.0));
    let p: PathElement<(i32, i32)> = PathElement::new(
        [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
        black_stroke(),
    );
    root.draw(&p)?;
    Ok(())
}

pub fn plot_stairs(
    root: &DrawingArea<BitMapBackend, Shift>,
    chart: &Chart,
    stair: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let drawing_area = chart.plotting_area();
    let mapped = drawing_area.map_coordinate(&(stair, 0.0));
    let p: PathElement<(i32, i32)> = PathElement::new(
        [(mapped.0, 0), (mapped.0, mapped.1)],
        lighter_stroke(),
    );
    root.draw(&p)?;
    Ok(())
}

pub fn plot_points(
    chart: &mut Chart,
    xs: &mut dyn Iterator<Item = &f64>,
    ys: DistIter<Uniform<f64>, ThreadRng, f64>,
    color: RGBColor,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    chart
        .draw_series(
            xs.zip(ys)
                .map(|(x, y)| Circle::new((*x, y), 2_i32, color.filled()))
                .choose_multiple(&mut rand::thread_rng(), 200),
        )?
        .label(label)
        .add_legend_icon(color);
    Ok(())
}

pub trait Ext {
    fn add_legend_icon(&mut self, color: RGBColor);
}

impl Ext for SeriesAnno<'_, BitMapBackend<'_>> {
    fn add_legend_icon(&mut self, color: RGBColor) {
        self.legend(move |(x, y)| {
            Rectangle::new([(x, y - 6), (x + 12, y + 6)], color.filled())
        });
    }
}
