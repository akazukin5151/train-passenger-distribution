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
    boarders_props: &[f64],
    i: usize,
    x: f64,
) -> f64 {
    let boarder_pdf = make_boarding_pdf_for_station(stations, i, x);
    if i == 0 {
        boarder_pdf
    } else {
        let boarders_as_prop_of_new = boarders_props[i];
        let remaining_pdf =
            make_pdf_for_station(stations, boarders_props, i - 1, x);
        let remaining_weighted_pdf =
            remaining_pdf * (1.0 - boarders_as_prop_of_new);
        let boarders_weighted_pdf = boarder_pdf * boarders_as_prop_of_new;
        remaining_weighted_pdf + boarders_weighted_pdf
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

pub fn calc_proportion_of_boarders(stations: &Vec<&str>) -> Vec<f64> {
    let link_loads = read_link_load_data();

    let line_loads: &Vec<_> = &link_loads
        .iter()
        .find(|(line, _)| line == "中央本線")
        .unwrap()
        .1
        .iter()
        .filter(|record| stations.contains(&&record[0]))
        .collect();

    // for the chuo line starting from tokyo, the direction is 'down'
    // if it is up then columns 4 and 5 will be used instead of 1 and 2
    let boardings = line_loads
        .iter()
        .map(|row| row[1].replace(',', "").parse::<i64>().unwrap());

    let alightings = line_loads
        .iter()
        .map(|row| row[2].replace(',', "").parse::<i64>().unwrap());

    // manually calculating cumulative here (even though (part of) it is already
    // in the third column, because some stations might have to be excluded)
    let difference = boardings.clone().zip(alightings).map(|(a, b)| a - b);
    let mut cumulative = vec![];
    for (idx, diff) in difference.enumerate() {
        let x = if idx == 0 { 0 } else { cumulative[idx - 1] };
        cumulative.push(diff + x)
    }

    // boarders as a percentage of total passengers in the train after the station
    let boarder_percs: Vec<_> = boardings
        .zip(cumulative)
        .map(|(boarding, cumulative)| {
            if cumulative == 0 {
                0.
            } else {
                boarding as f64 / cumulative as f64
            }
        })
        .collect();

    // let stations = line_loads.iter().map(|row| &row[0]);

    // first item is always 1 because 100% of passengers in the first station
    // are boarders; none of them were passengers remaining from a "previous" station
    // last item is always 0 because there are 0 passengers after the last station
    // "how many passengers out of a total of 0 passengers" == divide by zero
    // replaced with 0 to be consistent
    boarder_percs
}
