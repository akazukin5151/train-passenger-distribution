#![warn(clippy::all)]
mod data;
mod kde;
mod plot;
mod types;

use data::*;
use plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = generate_plot(generate_data())?;
    root.present()?;
    Ok(())
}
