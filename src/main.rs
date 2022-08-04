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
    plot_kde_separate(data.clone())?.present()?;
    plot_strip(data.clone())?.present()?;
    plot_kde_together(data)?.present()?;
    Ok(())
}
