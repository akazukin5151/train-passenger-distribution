use crate::kde::*;
use crate::types::*;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::ops::Range;

// This is a macro to avoid lifetime issues (ChartContext has a generic lifetime)
macro_rules! Chart {
    () => {
        ChartContext<
            BitMapBackend,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
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

fn black_stroke() -> ShapeStyle {
    ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    }
}

fn lighter_stroke() -> ShapeStyle {
    ShapeStyle {
        color: GREEN.mix(1.0),
        filled: true,
        stroke_width: 1,
    }
}

// This is a macro to avoid lifetime issues from CT due to root
macro_rules! plot_platform_bounds {
    ($chart:ident, $root:ident, $modifier:expr) => {
        let drawing_area = $chart.plotting_area();
        let mapped = drawing_area.map_coordinate(&(0.0, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - $modifier)],
            black_stroke(),
        );
        $root.draw(&p).unwrap();

        let mapped = drawing_area.map_coordinate(&(100.0, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - $modifier)],
            black_stroke(),
        );
        $root.draw(&p)?;
    };
}

// This is a macro to avoid recursive CT (CoordType) bounds requiring Deref
macro_rules! abstract_plot {
    (
        $out_file: expr,
        $y_range: expr,
        $n_stations:ident,
        $all_station_stairs:ident,
        $make_data:expr
    ) => {{
        let root =
            BitMapBackend::new($out_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let roots = root.split_evenly(($n_stations, 1));
        for (i, root) in roots.iter().enumerate() {
            let mut chart = basic_chart!(&root)
                .build_cartesian_2d::<Range<f64>, Range<f64>>(
                    -10.0..110.0,
                    $y_range,
                )?;

            let mut mesh = chart.configure_mesh();
            let mesh = mesh
                .y_desc(&$all_station_stairs[i].station_name)
                .axis_desc_style(
                    ("Hiragino Sans GB W3", 20_i32).into_text_style(root),
                )
                .light_line_style(&WHITE);
            if i == $n_stations - 1 {
                mesh.x_desc("xpos").draw()?;
            } else {
                mesh.draw()?;
            }

            $make_data(i, &mut chart);

            let modifier = 190 * i as i32;
            plot_platform_bounds!(chart, root, modifier);

            let drawing_area = chart.plotting_area();
            let stair_locations = &$all_station_stairs[i].stair_locations;
            for stair_location in stair_locations {
                let mapped =
                    drawing_area.map_coordinate(&(*stair_location as f64, 0.0));
                let p: PathElement<(i32, i32)> = PathElement::new(
                    [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
                    lighter_stroke(),
                );
                root.draw(&p)?;
            }
        }
        Ok(root)
    }};
}

pub fn plot_separate(
    (n_stations, all_station_stairs, train_passengers): (
        usize,
        Vec<StationStairs>,
        Vec<PassengerLocations>,
    ),
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    abstract_plot!(
        "out/out.png",
        0.0..0.06,
        n_stations,
        all_station_stairs,
        |i, chart: &mut Chart!()| {
            let tp: &PassengerLocations = &train_passengers[i];
            let res: Vec<_> = (0..=100)
                .map(|num| {
                    (
                        num as f64,
                        kernel_density_estimator(
                            &tp.passenger_locations,
                            scotts(tp.passenger_locations.len() as f64) * 12.0,
                            num as f64,
                        ),
                    )
                })
                .collect();
            chart
                .draw_series(LineSeries::new(res, BLUE.filled()))
                .unwrap();
        }
    )
}

pub fn plot_together(
    (_n_stations, all_station_stairs, train_passengers): (
        usize,
        Vec<StationStairs>,
        Vec<PassengerLocations>,
    ),
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    let root =
        BitMapBackend::new("out/together.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = basic_chart!(&root)
        .build_cartesian_2d(-10.0..110.0_f64, 0.0..0.06_f64)?;

    let mut mesh = chart.configure_mesh();
    mesh.y_desc("frequency")
        .axis_desc_style((20).into_text_style(&root))
        .light_line_style(&WHITE)
        .x_desc("xpos")
        .draw()?;

    let mut colors = (0..).map(Palette99::pick);
    for (this_station_stair, train_passenger) in
        all_station_stairs.iter().zip(train_passengers)
    {
        let res: Vec<_> = (0..=100)
            .map(|num| {
                (
                    num as f64,
                    kernel_density_estimator(
                        &train_passenger.passenger_locations,
                        scotts(train_passenger.passenger_locations.len() as f64)
                            * 12.0,
                        num as f64,
                    ),
                )
            })
            .collect();

        let style = colors.next().unwrap();
        chart
            .draw_series(LineSeries::new(res, style.filled()))?
            .label(&this_station_stair.station_name)
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 6), (x + 12, y + 6)], style.filled())
            });

        plot_platform_bounds!(chart, root, 0);
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(&BLACK.mix(0.5))
        .legend_area_size(22)
        .label_font(("Hiragino Sans GB W3", 20))
        .draw()?;

    Ok(root.clone())
}

pub fn plot_strip(
    (n_stations, all_station_stairs, train_passengers): (
        usize,
        Vec<StationStairs>,
        Vec<PassengerLocations>,
    ),
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    abstract_plot!(
        "out/strip.png",
        0.0..1.0,
        n_stations,
        all_station_stairs,
        |i, chart: &mut Chart!()| {
            let tp: &PassengerLocations = &train_passengers[i];
            let xs = &tp.passenger_locations;
            let uniform = Uniform::new(0.0, 1.0_f64);
            let ys = rand::thread_rng().sample_iter(uniform).take(xs.len());

            chart
                .draw_series(
                    xs.iter()
                        .zip(ys)
                        .map(|(x, y)| {
                            Circle::new((*x, y), 2_i32, BLUE.filled())
                        })
                        .choose_multiple(&mut rand::thread_rng(), 200),
                )
                .unwrap();
        }
    )
}
