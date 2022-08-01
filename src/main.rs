#![warn(clippy::all)]
mod data;
mod kde;
mod plot;

use plot::*;
use data::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = generate_plot(generate_data())?;
    root.present()?;
    Ok(())
}


