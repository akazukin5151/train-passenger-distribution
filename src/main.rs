#![warn(clippy::all)]
mod data;
mod plot;
mod types;

use data::*;
use plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations.clone());
    let boarder_props = calc_proportion_of_boarders(&stations);
    dbg!(&boarder_props);

    let pdfs = make_pdfs_for_all_stations(&all_station_stairs, &boarder_props);

    plot_pdfs("out/out.png", &all_station_stairs, pdfs.clone())?;
    plot_pdfs_together("out/together.png", &all_station_stairs, pdfs.clone())?;

    // TODO: duplicated computation. above should do it then combine it
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
