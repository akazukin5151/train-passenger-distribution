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
pub struct PassengerLocations {
    pub station_name: String,
    pub passenger_locations: Vec<f64>,
}
