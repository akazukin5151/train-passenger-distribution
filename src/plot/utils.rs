use crate::kde::*;
use crate::types::*;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;

// This is a macro to avoid lifetime issues (ChartContext has a generic lifetime)
macro_rules! Chart {
    () => {
        ChartContext<
            BitMapBackend,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64
            >,
        >
    }
}

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

pub const GRAY: RGBColor = RGBColor(100, 100, 100);

pub fn black_stroke() -> ShapeStyle {
    ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    }
}

pub fn lighter_stroke() -> ShapeStyle {
    ShapeStyle {
        color: GREEN.mix(1.0),
        filled: true,
        stroke_width: 1,
    }
}

pub fn plot_platform_bounds(
    chart: &ChartContext<
        BitMapBackend,
        Cartesian2d<RangedCoordf64, RangedCoordf64>,
    >,
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

pub fn make_kde(multiplier: f64, tp: &[f64]) -> Vec<(f64, f64)> {
    (0..=100)
        .map(|num| {
            let x = num as f64;
            let bandwidth = scotts(tp.len() as f64) * multiplier;
            let y = kernel_density_estimator(tp, bandwidth, x);
            (x, y)
        })
        .collect()
}
