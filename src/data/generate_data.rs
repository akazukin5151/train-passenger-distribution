use crate::data::read_data::*;
use crate::data::utils::*;
use crate::types::*;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::Distribution;

pub fn generate_passenger_locations(
    stations: Vec<&str>,
) -> (Vec<StationStairs>, Vec<PassengerLocations>) {
    let all_station_stairs: Vec<StationStairs> = stations
        .iter()
        .map(|station| StationStairs {
            station_name: station.to_string(),
            stair_locations: read_stair_locations(format!(
                "maps/{}.svg",
                station
            ))
            .unwrap(),
        })
        .collect();

    let od_pairs = read_od_row();

    let n_people = 200;
    let prop_normal_far = 0.6;
    let prop_uniform = 0.1;
    let prop_normal_close = 0.3;
    let n_normal_far = (f64::from(n_people) * prop_normal_far).floor();
    let n_uniform = (f64::from(n_people) * prop_uniform).floor();
    let n_normal_close = (f64::from(n_people) * prop_normal_close).floor();

    let far_stdev = 0.2;
    let close_stdev = 0.1;

    let mut train_passengers: Vec<PassengerLocations> = Vec::new();

    for this_station_stairs in all_station_stairs.clone() {
        let mut xs = generate_passenger_distribution(
            far_stdev,
            close_stdev,
            n_normal_far,
            n_normal_close,
            n_uniform,
            &this_station_stairs.stair_locations,
        );
        if train_passengers.is_empty() {
            train_passengers.push(PassengerLocations {
                passenger_locations: xs,
            });
        } else {
            let prev_row = train_passengers.last().unwrap();
            let prev_xs = &*prev_row.passenger_locations;
            let previous_stations: Vec<_> = all_station_stairs
                .iter()
                .map(|x| x.station_name.clone())
                .take_while(|x| *x != this_station_stairs.station_name)
                .collect();
            let passengers_aligning = od_pairs.iter().filter(|row| {
                let from_station = &row.from_station_code;
                let to_station = &row.to_station_code;
                let prevs = previous_stations.contains(from_station);
                (*to_station == this_station_stairs.station_name) && prevs
            });
            let n_passengers_aligning =
                passengers_aligning.fold(0, |acc, row| {
                    let count = row.count;
                    acc + count
                });
            dbg!(n_passengers_aligning);
            let n_passengers_in_train =
                prev_xs.len() + n_passengers_aligning as usize;
            let xs_remaining_from_prev = prev_xs.choose_multiple(
                &mut rand::thread_rng(),
                n_passengers_in_train,
            );
            xs.extend(xs_remaining_from_prev);
            train_passengers.push(PassengerLocations {
                passenger_locations: xs,
            });
        }
    }
    (all_station_stairs, train_passengers)
}

fn generate_passenger_distribution(
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
        let mean = clamp(*stair) / 100.0;
        let rand1: Vec<_> = if mean == 0.0 {
            FoldedNormal::new(0.3, 0.3)
                .sample_iter(&mut rng)
                .take(n_normal_far as usize)
                .map(|x| x * 100.0)
                .collect()
        } else {
            beta(mean, far_stdev)
                .unwrap()
                .sample_iter(&mut rng)
                .take(n_normal_far as usize)
                .map(|x| x * 100.0)
                .collect()
        };
        let rand2: Vec<_> = if mean == 0.0 {
            FoldedNormal::new(0.3, 0.3)
                .sample_iter(&mut rng)
                .take(n_normal_close as usize)
                .map(|x| x * 100.0)
                .collect()
        } else {
            beta(mean, close_stdev)
                .unwrap()
                .sample_iter(&mut rng)
                .take(n_normal_close as usize)
                .map(|x| x * 100.0)
                .collect()
        };
        let rand3: Vec<_> = rng
            .clone()
            .sample_iter(uniform)
            .take(n_uniform as usize)
            .collect();
        xs.extend(rand1);
        xs.extend(rand2);
        xs.extend(rand3);
    }
    xs
}
