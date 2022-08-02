use crate::kde::*;
use crate::types::*;
use plotters::coord::Shift;
use plotters::prelude::*;

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
    let root =
        BitMapBackend::new("out/out.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let black_stroke = ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    };

    let lighter_stroke = ShapeStyle {
        color: GREEN.mix(1.0),
        filled: true,
        stroke_width: 1,
    };

    let roots = root.split_evenly((n_stations, 1));
    for (i, root) in roots.iter().enumerate() {
        let mut chart = ChartBuilder::on(root)
            .margin_left(10)
            .margin_right(30)
            .margin_top(10)
            .margin_bottom(10)
            .x_label_area_size(40_i32)
            .y_label_area_size(80_i32)
            .build_cartesian_2d(-10.0..110.0, 0.0..0.06)?;

        let mut mesh = chart.configure_mesh();
        let mesh = mesh
            .y_desc(&all_station_stairs[i].station_name)
            .axis_desc_style(("Hiragino Sans GB W3", 20).into_text_style(root))
            .light_line_style(&WHITE);
        if i == n_stations - 1 {
            mesh.x_desc("xpos").draw()?;
        } else {
            mesh.draw()?;
        }

        let res: Vec<_> = (0..=100)
            .map(|num| {
                (
                    num as f64,
                    kernel_density_estimator(
                        &train_passengers[i].passenger_locations,
                        scotts(train_passengers[i].passenger_locations.len()
                            as f64)
                            * 12.0,
                        num as f64,
                    ),
                )
            })
            .collect();

        chart.draw_series(LineSeries::new(res, BLUE.filled()))?;

        let drawing_area = chart.plotting_area();

        let mapped = drawing_area.map_coordinate(&(0.0, 0.0));
        let modifier = 190 * i as i32;
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
            black_stroke,
        );
        root.draw(&p)?;

        let mapped = drawing_area.map_coordinate(&(100.0, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
            black_stroke,
        );
        root.draw(&p)?;

        let stair_locations = &all_station_stairs[i].stair_locations;
        for stair_location in stair_locations {
            let mapped =
                drawing_area.map_coordinate(&(*stair_location as f64, 0.0));
            let p: PathElement<(i32, i32)> = PathElement::new(
                [(mapped.0, 0), (mapped.0, mapped.1 - modifier)],
                lighter_stroke,
            );
            root.draw(&p)?;
        }
    }
    Ok(root)
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

    let black_stroke = ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    };

    let lighter_stroke = ShapeStyle {
        color: GREEN.mix(1.0),
        filled: true,
        stroke_width: 1,
    };

    let mut chart = ChartBuilder::on(&root)
        .margin_left(10)
        .margin_right(30)
        .margin_top(10)
        .margin_bottom(10)
        .x_label_area_size(40_i32)
        .y_label_area_size(80_i32)
        .build_cartesian_2d(-10.0..110.0, 0.0..0.06)?;

    let mut mesh = chart.configure_mesh();
    mesh.y_desc("frequency")
        .axis_desc_style(("Hiragino Sans GB W3", 20).into_text_style(&root))
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

        let drawing_area = chart.plotting_area();

        let mapped = drawing_area.map_coordinate(&(0.0, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1)],
            black_stroke,
        );
        root.draw(&p)?;

        let mapped = drawing_area.map_coordinate(&(100.0, 0.0));
        let p: PathElement<(i32, i32)> = PathElement::new(
            [(mapped.0, 0), (mapped.0, mapped.1)],
            black_stroke,
        );
        root.draw(&p)?;

        //let stair_locations = &this_station_stair.stair_locations;
        //for stair_location in stair_locations {
        //    let mapped =
        //        drawing_area.map_coordinate(&(*stair_location as f64, 0.0));
        //    let p: PathElement<(i32, i32)> = PathElement::new(
        //        [(mapped.0, 0), (mapped.0, mapped.1)],
        //        lighter_stroke,
        //    );
        //    root.draw(&p)?;
        //}
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(&BLACK.mix(0.5))
        .legend_area_size(22)
        .label_font("Hiragino Sans GB W3")
        .draw()?;

    Ok(root.clone())
}
