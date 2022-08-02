use crate::types::OdRow;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;
use std::cmp::Ordering;
use std::ops::Deref;
use svg::parser::Event;

fn read_data_from_file(
    path: String,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let events = svg::open(path, &mut content)?;
    let mut guideline_pos: Vec<f64> = events
        .filter_map(|event| {
            if let Event::Tag(path, _, attrs) = event {
                if path == "sodipodi:guide" {
                    let raw_pos = attrs.get("position").unwrap();
                    let pos = raw_pos.deref();
                    // to_string() turn &str into String, which create
                    // a new allocation and own the data, because the &str pointer
                    // would be dropped at the end
                    let xpos_str = pos.split(',').next().unwrap().to_string();
                    let xpos: f64 = xpos_str.parse().unwrap();
                    if !xpos.is_nan() {
                        return Some(xpos);
                    }
                };
            };
            None
        })
        .collect();

    guideline_pos.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let max = guideline_pos.pop().unwrap();
    // This is O(n) but still more efficient than using VecDeque, because it needs
    // `.make_contiguous().sort_by()` to sort and then `buf.as_slices()` to convert back
    // to vec
    let min = guideline_pos.remove(0);

    let result = guideline_pos
        .iter()
        .map(|x| (x - min) / (max - min) * 100.0)
        .collect();
    Ok(result)
}

fn read_od_row() -> Vec<OdRow> {
    let mut rdr = csv::Reader::from_path("data/out.csv").unwrap();
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Result<OdRow, _> = result;
        if let Ok(r) = record {
            records.push(r);
        }
    }
    records
}

pub fn generate_data(
    stations: Vec<&str>,
) -> (usize, Vec<(String, Vec<f64>)>, Vec<(String, Vec<f64>)>) {
    let n_stations = stations.len();
    let station_stairs: Vec<(String, Vec<f64>)> = stations
        .iter()
        .map(|station| {
            (
                station.to_string(),
                read_data_from_file(format!("maps/{}.svg", station)).unwrap(),
            )
        })
        .collect();

    // TODO: add OD pairs from dataset
    let rows = read_od_row();
    let od_pairs: Vec<_> = rows
        .iter()
        .map(|x| ((&x.from_station_code, &x.to_station_code), &x.count))
        .collect();

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
                *to_station == station_name
            });
            let n_passengers_aligning =
                passengers_aligning.fold(0, |acc, row| {
                    let count = row.1;
                    acc + count
                });
            let n_passengers_in_train =
                prev_xs.len() + n_passengers_aligning as usize;
            let xs_remaining_from_prev = prev_xs.choose_multiple(
                &mut rand::thread_rng(),
                n_passengers_in_train,
            );
            xs.extend(xs_remaining_from_prev);
            train_passengers.push((station_name, xs));
        }
    }
    (n_stations, station_stairs, train_passengers)
}

fn station(
    far_stdev: f64,
    close_stdev: f64,
    n_normal_far: f64,
    n_normal_close: f64,
    n_uniform: f64,
    stair_locations: &[f64],
) -> Vec<f64> {
    let mut xs = Vec::new();
    let mut rng = rand::thread_rng();
    for stair in stair_locations {
        let uniform = Uniform::new(0.0, 100.0);
        let rand1 = Normal::new(*stair, far_stdev)
            .unwrap()
            .sample_iter(&mut rng)
            .take(n_normal_far as usize)
            .collect();
        let rand2 = Normal::new(*stair, close_stdev)
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
