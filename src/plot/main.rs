use crate::plot::utils::*;
use crate::types::*;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::ops::Range;

pub fn plot_kde_separate(
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
            let res = make_kde(tp);
            chart
                .draw_series(LineSeries::new(res, BLUE.filled()))
                .unwrap();
        }
    )
}

pub fn plot_kde_together(
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
        let res = make_kde(&train_passenger);
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

