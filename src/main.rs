#![warn(clippy::all)]
mod data;
mod kde;
mod plot;
mod types;

use data::*;
use plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let all_station_stairs = read_station_stairs(stations);
    let data = generate_boarding_distributions(&all_station_stairs);
    let od_pairs = read_od_row();
    let n_passengers_alighting =
        make_cumulative(1, all_station_stairs, od_pairs);
    //let data = generate_passenger_locations(stations);
    //plot_kde_separate(&data, 12.0)?.present()?;
    //plot_strip(&data)?.present()?;
    //plot_kde_together(&data, "out/together.png", 12.0)?.present()?;
    //plot_kde_together(&data, "out/smoothed.png", 25.0)?.present()?;
    plot_step_by_step(
        n_passengers_alighting,
        &data,
        "out/step-by-step.png",
        12.0,
    )?
    .present()?;
    Ok(())
}
