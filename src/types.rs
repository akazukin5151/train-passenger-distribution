use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OdRow {
    pub index: i32,
    pub from_station_code: String,
    pub to_station_code: String,
    pub count: i64,
    pub stations: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct StationStairs {
    pub station_name: String,
    pub stair_locations: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct BoardingData {
    pub stair_location: f64,
    pub beta_far: Vec<f64>,
    pub beta_close: Vec<f64>,
    pub uniform: Vec<f64>,
}

pub type Accumulator = Vec<(
    // boarding_data,
    Vec<BoardingData>,
    // n_passengers_alighting,
    i64,
    // remaining_xs,
    Vec<f64>,
    // all_xs,
    Vec<f64>,
)>;
