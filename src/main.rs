use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;
use std::f64::consts::E;
use std::f64::consts::PI;

fn scotts(n: f64) -> f64 {
    n.powf(-1.0 / (1.0 + 4.0))
}

fn gaussian(u: f64) -> f64 {
    1.0 / ((2.0 * PI).powi(1 / 2)) * E.powf((-1.0 / 2.0) * u.powi(2))
}

fn kernel_density_estimator(xs: &[f64], bandwidth: f64, x: f64) -> f64 {
    let n = xs.len() as f64;
    let summed: f64 = xs.iter().map(|xi| gaussian((x - xi) / bandwidth)).sum();
    (1.0 / (n * bandwidth)) * summed
}

fn clamp(xs: Vec<f64>) -> Vec<f64> {
    xs.iter()
        .map(|x| {
            if *x >= 0.0 && *x <= 100.0 {
                *x
            } else if *x > 100.0 {
                100.0
            } else {
                0.0
            }
        })
        .collect()
}

fn station(
    far_stdev: f64,
    close_stdev: f64,
    n_normal_far: f64,
    n_normal_close: f64,
    n_uniform: f64,
    stair_locations: &[i32],
) -> Vec<f64> {
    let mut xs = Vec::new();
    let mut rng = rand::thread_rng();
    for stair in stair_locations {
        let uniform = Uniform::new(0.0, 100.0);
        let rand1 = Normal::new(f64::from(*stair), far_stdev)
            .unwrap()
            .sample_iter(&mut rng)
            .take(n_normal_far as usize)
            .collect();
        let rand2 = Normal::new(f64::from(*stair), close_stdev)
            .unwrap()
            .sample_iter(&mut rng)
            .take(n_normal_close as usize)
            .collect();
        let rand3 = rng
            .clone()
            .sample_iter(uniform)
            .take(n_uniform as usize)
            .collect();
        xs.extend(clamp(rand1));
        xs.extend(clamp(rand2));
        xs.extend(clamp(rand3));
    }
    xs
}

const OUT_FILE_NAME: &'static str = "out/out.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let station_stairs = [
        ("0".to_string(), vec![30, 70]),
        ("1".to_string(), vec![50]),
        ("2".to_string(), vec![10]),
    ];
    let n_stations = station_stairs.len();

    let od_pairs = [
        (("0".to_string(), "1"), 10),
        (("0".to_string(), "2"), 10),
        (("1".to_string(), "2"), 10),
    ];

    let n_people = 50;
    let prop_normal_far = 0.6;
    let prop_uniform = 0.1;
    let prop_normal_close = 0.3;
    let n_normal_far = (f64::from(n_people) * prop_normal_far).floor();
    let n_uniform = (f64::from(n_people) * prop_uniform).floor();
    let n_normal_close = (f64::from(n_people) * prop_normal_close).floor();

    let far_stdev = 20.0;
    let close_stdev = 10.0;

    let mut train_passengers: Vec<(String, Vec<f64>)> = Vec::new();

    for (station_name, stair_locations) in station_stairs.clone() {
        let mut xs = station(
            far_stdev,
            close_stdev,
            n_normal_far,
            n_normal_close,
            n_uniform,
            &stair_locations,
        );
        if train_passengers.is_empty() {
            train_passengers.push((station_name, xs));
        } else {
            let prev_row = train_passengers.last().unwrap();
            let prev_xs = &*prev_row.1;
            let passengers_aligning = od_pairs.iter().filter(|row| {
                let to_station = row.0 .1;
                to_station == station_name
            });
            let n_passengers_aligning =
                passengers_aligning.fold(0, |acc, row| {
                    let count = row.1;
                    acc + count
                });
            let n_passengers_in_train = prev_xs.len() + n_passengers_aligning;
            let xs_remaining_from_prev = prev_xs.choose_multiple(
                &mut rand::thread_rng(),
                n_passengers_in_train,
            );
            xs.extend(xs_remaining_from_prev);
            train_passengers.push((station_name, xs));
        }
    }

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
        let mut chart = ChartBuilder::on(&root)
            .margin_left(10)
            .margin_right(30)
            .margin_top(10)
            .margin_bottom(10)
            .x_label_area_size(40_i32)
            .y_label_area_size(80_i32)
            .build_cartesian_2d(0.0..100.0, 0.0..0.1)?;

        if i == n_stations - 1 {
            chart
                .configure_mesh()
                .y_desc(&station_stairs[i].0)
                .light_line_style(&WHITE)
                .x_desc("xpos")
                .draw()?;
        } else {
            chart
                .configure_mesh()
                .light_line_style(&WHITE)
                .y_desc(&station_stairs[i].0)
                .draw()?;
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
        let modifier = 250 * i as i32;
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

        let (_, stair_locations) = &station_stairs[i];
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

    root.present()?;
    Ok(())
}
