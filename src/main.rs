use plotters::data::fitting_range;
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::f64::consts::E;
use std::f64::consts::PI;

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

const OUT_FILE_NAME: &'static str = "boxplot.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let station_stairs = [
        ("0".to_string(), vec![30, 70]),
        ("1".to_string(), vec![50]),
        ("2".to_string(), vec![10]),
    ];

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

    let ds: HashMap<(String, String), Vec<f64>> =
        HashMap::from_iter(train_passengers.iter().map(|(station, xposes)| {
            ((station.clone(), "".to_string()), xposes.clone())
        }));

    let dataset: Vec<(String, String, _)> = ds
        .iter()
        .map(|(k, v)| (k.0.clone(), k.1.clone(), v.clone()))
        .collect();

    let host_list: Vec<String> = station_stairs
        .iter()
        .map(|(x, _)| x.clone())
        .rev()
        .collect();

    let mut colors = (0..).map(Palette99::pick);
    let mut offsets = (-12..).step_by(24);
    let mut series = BTreeMap::new();
    for x in dataset.iter() {
        let entry = series.entry(x.1.clone()).or_insert_with(|| {
            (Vec::new(), colors.next().unwrap(), offsets.next().unwrap())
        });
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f64> = dataset.iter().map(|x| x.2).flatten().collect();
    let values_range = fitting_range(values.iter());

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40_i32)
        .y_label_area_size(80_i32)
        .build_cartesian_2d(
            values_range.start - 1.0..values_range.end + 1.0,
            values_range.start - 1.0..values_range.end + 1.0,
            //host_list[..].into_segmented(),
        )?;

    chart
        .configure_mesh()
        .x_desc("xpos")
        .y_desc("frequency")
        .y_labels(host_list.len())
        .light_line_style(&WHITE)
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..=100)
            .map(|num| kernel_density_estimator(series[0].0.iter().map(|(_, x) x).collect(), 1.0, num as f64)),
        style,
    ));
    for (label, (values, style, offset)) in &series {
        (values.iter().map(|x| {
            chart.draw_series(LineSeries::new(
                (0..=100)
                    .map(|num| kernel_density_estimator(x.1, 1.0, num as f64)),
                style,
            ))
            //Boxplot::new_horizontal(SegmentValue::CenterOf(&x.0), &x.1)
            //    .width(20)
            //    .whisker_width(0.5)
            //    .style(style)
            //    .offset(*offset)
        }));
    }

    let black_stroke = ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    };

    let lighter_stroke = ShapeStyle {
        color: BLUE.mix(1.0),
        filled: true,
        stroke_width: 1,
    };

    //let drawing_area = chart.plotting_area();
    //let mapped_x_0 = drawing_area.map_coordinate(&(0.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x_0, 0),
    //        (mapped_x_0, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    black_stroke,
    //);
    //root.draw(&p)?;

    //let mapped_x = drawing_area.map_coordinate(&(100.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x, 0),
    //        (mapped_x, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    black_stroke,
    //);
    //root.draw(&p)?;

    //let mapped_x = drawing_area.map_coordinate(&(30.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x, 0),
    //        (mapped_x, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    lighter_stroke,
    //);
    //root.draw(&p)?;

    //let mapped_x = drawing_area.map_coordinate(&(70.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x, 0),
    //        (mapped_x, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    lighter_stroke,
    //);
    //root.draw(&p)?;

    //let mapped_x = drawing_area.map_coordinate(&(50.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x, 0),
    //        (mapped_x, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    lighter_stroke,
    //);
    //root.draw(&p)?;

    //let mapped_x = drawing_area.map_coordinate(&(10.0, SegmentValue::Last)).0;
    //let p: PathElement<(i32, i32)> = PathElement::new(
    //    [
    //        (mapped_x, 0),
    //        (mapped_x, drawing_area.get_y_axis_pixel_range().end + 1),
    //    ],
    //    lighter_stroke,
    //);
    //root.draw(&p)?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
