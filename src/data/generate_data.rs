use crate::data::read_data::*;
use crate::data::utils::*;
use crate::types::*;
use rand::distributions::Uniform;
use rand::Rng;
use rand_distr::Distribution;

fn stairs_to_beta(far_stdev: f64, n_normal_far: f64, stair: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mean = clamp(stair) / 100.0;
    beta(mean, far_stdev)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n_normal_far as usize)
        .map(|x| x * 100.0)
        .collect()
}

fn stairs_to_uniform(n_uniform: f64) -> Vec<f64> {
    let rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, 100.0);
    rng.sample_iter(uniform).take(n_uniform as usize).collect()
}

pub fn read_station_stairs(stations: Vec<&str>) -> Vec<StationStairs> {
    stations
        .iter()
        .map(|station| StationStairs {
            station_name: station.to_string(),
            stair_locations: read_stair_locations(format!(
                "maps/{}.svg",
                station
            ))
            .unwrap(),
        })
        .collect()
}

pub fn generate_boarding_distributions(
    all_station_stairs: &[StationStairs],
) -> Vec<Vec<BoardingData>> {
    let n_people = 200;
    let prop_normal_far = 0.6;
    let prop_uniform = 0.1;
    let prop_normal_close = 0.3;
    let n_normal_far = (f64::from(n_people) * prop_normal_far).floor();
    let n_uniform = (f64::from(n_people) * prop_uniform).floor();
    let n_normal_close = (f64::from(n_people) * prop_normal_close).floor();

    let far_stdev = 0.2;
    let close_stdev = 0.1;

    all_station_stairs
        .iter()
        .map(|this_station_stairs| {
            this_station_stairs
                .stair_locations
                .iter()
                .map(|stair_location| {
                    let far = stairs_to_beta(
                        far_stdev,
                        n_normal_far,
                        *stair_location,
                    );
                    let close = stairs_to_beta(
                        close_stdev,
                        n_normal_close,
                        *stair_location,
                    );
                    let uniform = stairs_to_uniform(n_uniform);
                    BoardingData {
                        stair_location: *stair_location,
                        beta_far: far,
                        beta_close: close,
                        uniform,
                    }
                })
                .collect()
        })
        .collect()
}

pub fn get_n_alighting(
    index: usize,
    all_station_stairs: &[StationStairs],
    od_pairs: &[OdRow],
) -> i64 {
    let this_station_stairs = &all_station_stairs[index];
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
    let n_passengers_alighting = passengers_aligning.fold(0, |acc, row| {
        let count = row.count;
        acc + count
    });
    dbg!(n_passengers_alighting);
    n_passengers_alighting
}
