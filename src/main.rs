#![warn(clippy::all)]
mod data;
mod plot;
mod types;

use data::*;
use plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations);
    let od_pairs = read_od_row();

    let pdfs: Vec<Vec<(f64, f64)>> = all_station_stairs
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            (1..=100)
                .map(|x| {
                    let y = make_pdf_for_station(
                        &all_station_stairs,
                        // TODO
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

    plot_pdfs("out/out.png", &all_station_stairs, pdfs.clone())?;
    plot_pdfs_together("out/together.png", &all_station_stairs, pdfs.clone())?;

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
        "out/step-by-step.png",
        x,
        &all_station_stairs[2].stair_locations,
        prev_pdf,
        this_pdf,
    )?;

    Ok(())
}
