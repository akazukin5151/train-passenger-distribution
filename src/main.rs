#![warn(clippy::all)]
mod data;
mod kde;
mod plot;
mod types;
mod utils;

use crate::types::OdRow;
use crate::types::StationStairs;
use data::*;
use plot::*;
use rand::prelude::SliceRandom;
use utils::*;

fn combine_all(
    data: &Vec<Vec<(f64, Vec<f64>, Vec<f64>, Vec<f64>)>>,
    all_station_stairs: &Vec<StationStairs>,
    od_pairs: &Vec<OdRow>,
) -> Vec<Vec<f64>> {
    let tokyo = data[0].clone();
    let tokyo_xs = sum_boarding_types(&tokyo);

    data.iter()
        .skip(1)
        .fold((1, vec![tokyo_xs]), |(index, mut acc), boarding_data| {
            let prev: &Vec<f64> = &acc[index - 1];

            let n_passengers_alighting =
                get_n_alighting(index, all_station_stairs, od_pairs.clone());

            let n_passengers_remaining =
                prev.len() - (n_passengers_alighting as usize);

            dbg!(n_passengers_remaining);

            let remaining_xs = prev.choose_multiple(
                &mut rand::thread_rng(),
                n_passengers_remaining,
            );

            let boarding_xs = sum_boarding_types(boarding_data);

            let all_xs: Vec<f64> =
                remaining_xs.cloned().chain(boarding_xs).collect();

            acc.push(all_xs);

            (index + 1, acc)
        })
        .1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations);
    let data = generate_boarding_distributions(&all_station_stairs);
    let od_pairs = read_od_row();

    let n_passengers_alighting =
        get_n_alighting(1, &all_station_stairs, od_pairs.clone());

    let (r, kanda_tp) = plot_step_by_step(
        n_passengers_alighting,
        &data,
        "out/step-by-step.png",
        12.0,
    )?;
    r.present()?;

    let mut tp = combine_all(&data, &all_station_stairs, &od_pairs);
    // the tokyo distribution is apparently the same
    tp[1] = kanda_tp;

    plot_kde_separate(&all_station_stairs, &tp, 12.0)?.present()?;
    plot_strip(&all_station_stairs, &tp)?.present()?;
    plot_kde_together(&all_station_stairs, &tp, "out/together.png", 12.0)?
        .present()?;
    plot_kde_together(&all_station_stairs, &tp, "out/smoothed.png", 25.0)?
        .present()?;

    Ok(())
}
