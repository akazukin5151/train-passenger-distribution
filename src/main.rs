#![warn(clippy::all)]
mod data;
mod kde;
mod plot;
mod types;
mod utils;

use crate::types::*;
use data::*;
use plot::*;
use rand::prelude::SliceRandom;
use utils::*;

fn combine_all(
    all_boarding_data: &Vec<Vec<BoardingData>>,
    all_station_stairs: &[StationStairs],
    od_pairs: &[OdRow],
    tokyo_xs: Vec<f64>,
) -> Accumulator {
    let tokyo_row =
        (all_boarding_data[0].clone(), 0, tokyo_xs.clone(), tokyo_xs);
    all_boarding_data.iter().skip(1).fold(
        vec![tokyo_row],
        |mut acc, boarding_data| {
            let acc_len = acc.len();
            let nth_station = acc_len;
            let index_of_last = acc_len - 1;
            let (_, _, _, prev) = acc[index_of_last].clone();

            let n_passengers_alighting =
                get_n_alighting(nth_station, all_station_stairs, od_pairs);

            let n_passengers_remaining =
                prev.len() - (n_passengers_alighting as usize);

            dbg!(n_passengers_remaining);

            let remaining_xs = prev.choose_multiple(
                &mut rand::thread_rng(),
                n_passengers_remaining,
            );

            let boarding_xs = sum_boarding_types(boarding_data);

            let y: Vec<_> = remaining_xs.cloned().collect();
            let all_xs: Vec<f64> =
                y.iter().cloned().chain(boarding_xs).collect();

            acc.push((
                boarding_data.to_vec(),
                n_passengers_alighting,
                y,
                all_xs,
            ));

            acc.to_vec()
        },
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations);
    let data = generate_boarding_distributions(&all_station_stairs);
    let od_pairs = read_od_row();

    let tokyo = &data[0];
    let tokyo_xs = sum_boarding_types(tokyo);

    let result = combine_all(&data, &all_station_stairs, &od_pairs, tokyo_xs);

    let tp: Vec<Vec<f64>> = result.iter().map(|x| x.3.clone()).collect();

    // the tokyo distribution is apparently the same

    plot_kde_separate(&all_station_stairs, &tp, 12.0)?.present()?;
    plot_strip(&all_station_stairs, &tp)?.present()?;
    plot_kde_together(&all_station_stairs, &tp, "out/together.png", 12.0)?
        .present()?;
    plot_kde_together(&all_station_stairs, &tp, "out/smoothed.png", 25.0)?
        .present()?;

    plot_step_by_step(&result, "out/step-by-step.png", 12.0)?.present()?;

    Ok(())
}
