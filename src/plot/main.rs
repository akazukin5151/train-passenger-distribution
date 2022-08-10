use crate::plot::utils::*;
use crate::types::*;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::ops::Range;

pub fn plot_kde_separate(
    (all_station_stairs, train_passengers): (
        Vec<StationStairs>,
        Vec<Vec<Vec<(f64, f64)>>>,
    ),
) -> Result<
    DrawingArea<BitMapBackend<'static>, Shift>,
    Box<dyn std::error::Error>,
> {
    abstract_plot!(
        "out/out.png",
        0.0..15.0,
        all_station_stairs,
        |i: usize, chart: &mut Chart!()| {
            let tp: Vec<Vec<(f64, f64)>> = train_passengers[i].clone();
            assert_eq!(train_passengers.len(), all_station_stairs.len());
            assert_eq!(tp.len(), all_station_stairs[i].stair_locations.len());

            let combined: Vec<(f64, f64)> = (1..=999).map(|i| {
                let mut res = 0.0;
                for j in 0..tp.len() {
                    let a = tp[j][i].1;
                    if a != f64::INFINITY {
                        res += a;
                    }
                }
                (tp[0][i].0 * 100.0, res)
            }).collect();
            chart
                .draw_series(LineSeries::new(combined, BLUE.filled()))
                .unwrap();

            //for ts in tp {
            //    let filtered: Vec<(f64, f64)> = ts
            //        .iter()
            //        .filter(|(_, y)| *y != f64::INFINITY)
            //        .cloned()
            //        .collect();
            //    chart
            //        .draw_series(LineSeries::new(filtered, BLUE.filled()))
            //        .unwrap();
            //}
        }
    )
}

pub fn plot_kde_together(
    (all_station_stairs, train_passengers): (
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

    let colors = [
        RGBColor(76, 114, 176),
        RGBColor(221, 132, 82),
        RGBColor(85, 168, 104),
        RGBColor(196, 78, 82),
    ];
    for ((this_station_stair, train_passenger), color) in
        all_station_stairs.iter().zip(train_passengers).zip(colors)
    {
        let res = make_kde(&train_passenger);
        chart
            .draw_series(LineSeries::new(res, color.stroke_width(3)))?
            .label(&this_station_stair.station_name)
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 6), (x + 12, y + 6)], color.filled())
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
    (all_station_stairs, train_passengers): (
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
