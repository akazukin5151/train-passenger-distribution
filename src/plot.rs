use crate::types::*;
use crate::kde::*;
use plotters::coord::Shift;
use plotters::prelude::*;

pub fn generate_plot(
    (n_stations, station_stairs, train_passengers): (
        usize,
        Vec<StationStairs>,
        Vec<(String, Vec<f64>)>,
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
            .build_cartesian_2d(-10.0..110.0, 0.0..0.07)?;

        let mut mesh = chart.configure_mesh();
        let mesh = mesh
            .y_desc(&station_stairs[i].station_name)
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
                        &train_passengers[i].1,
                        scotts(train_passengers[i].1.len() as f64) * 12.0,
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

        let stair_locations = &station_stairs[i].stair_locations;
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
