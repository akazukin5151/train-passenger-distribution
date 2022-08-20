use crate::data::read_data::*;
use crate::data::utils::*;
use crate::types::*;
use statrs::distribution::Continuous;

// m
//#[cached(
//    type = "UnboundCache<(usize, i32), f64>",
//    create = "{ UnboundCache::new() }",
//    convert = " { (i, x) } "
//)]
pub fn make_pdf_for_station(
    stations: &[StationStairs],
    alighting_proportions: &[f64],
    i: usize,
    x: f64,
) -> f64 {
    let common = make_boarding_pdf_for_station(stations, i, x);
    if i == 0 {
        common
    } else {
        let proportion_alighting = alighting_proportions[i];
        let a = make_pdf_for_station(stations, alighting_proportions, i - 1, x)
            * (1.0 - proportion_alighting);
        let b = common * proportion_alighting;
        a + b
    }
}

// b
pub fn make_boarding_pdf_for_station(
    stations: &[StationStairs],
    i: usize,
    x: f64,
) -> f64 {
    stations[i]
        .stair_locations
        .iter()
        .map(|stair| {
            stair_pdfs(stair, x) / stations[i].stair_locations.len() as f64
        })
        .sum()
}

// S
pub fn stair_pdfs(stair: &f64, x: f64) -> f64 {
    let (a, b, c) = stair_pdfs_sep(stair, x);
    a + b + c
}

pub fn stair_pdfs_sep(stair: &f64, x: f64) -> (f64, f64, f64) {
    let prop_normal_far = 0.6;
    let prop_uniform = 0.1;
    let prop_normal_close = 0.3;

    let far_concentration = 7.;
    let close_concentration = 20.;

    let mean = clamp(*stair) / 100.0;
    let a = beta(mean, far_concentration, x) * prop_normal_far;
    let b = beta(mean, close_concentration, x) * prop_normal_close;
    let c = statrs::distribution::Uniform::new(0.0, 1.).unwrap().pdf(x)
        * prop_uniform;

    (a, b, c)
}

pub fn read_station_stairs(stations: Vec<&str>) -> Vec<StationStairs> {
    stations
        .iter()
        .map(|station| StationStairs {
            station_name: station.to_string(),
            stair_locations: read_stair_locations(format!(
                "maps/{}.svg",
                station
            ))
            .unwrap(),
        })
        .collect()
}
