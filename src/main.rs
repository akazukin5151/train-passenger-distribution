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
) -> Vec<Accumulator> {
    let tokyo_row = Accumulator {
        boarding_data: all_boarding_data[0].clone(),
        n_passengers_alighting: 0,
        remaining_xs: tokyo_xs.clone(),
        all_xs: tokyo_xs,
    };
    all_boarding_data.iter().skip(1).fold(
        vec![tokyo_row],
        |mut acc, boarding_data| {
            let acc_len = acc.len();
            let nth_station = acc_len;
            let index_of_last = acc_len - 1;
            let prev = acc[index_of_last].clone().all_xs;

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

            acc.push(Accumulator {
                boarding_data: boarding_data.to_vec(),
                n_passengers_alighting,
                remaining_xs: y,
                all_xs,
            });

            acc.to_vec()
        },
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations);
    //let data = generate_boarding_distributions(&all_station_stairs);
    //let od_pairs = read_od_row();

    let pdfs: Vec<Vec<(f64, f64)>> = all_station_stairs
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            (1..=100)
                .map(|x| {
                    let y = make_pdf_for_station(
                        &all_station_stairs,
                        &std::iter::repeat(0.5_f64)
                            .take(all_station_stairs.len())
                            .collect::<Vec<_>>(),
                        idx,
                        x as f64 / 100.0,
                    );
                    (x as f64, y)
                })
                .collect()
        })
        .collect();

    plot_pdfs("out/out2.png", &all_station_stairs, pdfs.clone())?;

    let x: Vec<Vec<(f64, (f64, f64, f64))>> = (1..=100)
        .map(|x| {
            all_station_stairs[2]
                .stair_locations
                .iter()
                .map(|stair| {
                    let div =
                        all_station_stairs[2].stair_locations.len() as f64;
                    let (a, b, c) = stair_pdfs_sep(stair, x as f64 / 100.);
                    (x as f64, (a / div, b / div, c / div))
                })
                .collect()
        })
        .collect();

    let prev_pdf = &pdfs[1];
    let this_pdf = &pdfs[2];

    plot_stair_pdfs_sep(
        "out/step-by-step2.png",
        x,
        &all_station_stairs[2].stair_locations,
        prev_pdf,
        this_pdf,
    )?;

    //let tokyo = &data[0];
    //let tokyo_xs = sum_boarding_types(tokyo);

    //let result = combine_all(&data, &all_station_stairs, &od_pairs, tokyo_xs);

    //let tp: Vec<Vec<f64>> = result.iter().map(|x| x.all_xs.clone()).collect();

    //// the tokyo distribution is apparently the same

    //plot_kde_separate(&all_station_stairs, &tp, 12.0)?.present()?;
    //plot_strip(&all_station_stairs, &tp)?.present()?;
    //plot_kde_together(&all_station_stairs, &tp, "out/together.png", 12.0)?
    //    .present()?;
    //plot_kde_together(&all_station_stairs, &tp, "out/smoothed.png", 25.0)?
    //    .present()?;

    //plot_step_by_step(&result, "out/step-by-step.png", 12.0)?.present()?;

    Ok(())
}
