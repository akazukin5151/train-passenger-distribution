use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;

pub fn generate_data() -> (usize, Vec<(String, Vec<i32>)>, Vec<(String, Vec<f64>)>)
{
    let station_stairs = vec![
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
    (n_stations, station_stairs, train_passengers)
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

