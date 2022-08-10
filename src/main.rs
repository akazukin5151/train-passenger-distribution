#![warn(clippy::all)]
mod data;
mod kde;
mod plot;
mod types;

use data::*;
use plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stations = vec!["東京", "神田", "御茶ノ水", "四ッ谷"];
    let data = generate_passenger_locations(stations);
    plot_kde_separate(&data, 12.0)?.present()?;
    plot_strip(&data)?.present()?;
    plot_kde_together(&data, "out/together.png", 12.0)?.present()?;
    plot_kde_together(&data, "out/smoothed.png", 25.0)?.present()?;
    Ok(())
}
