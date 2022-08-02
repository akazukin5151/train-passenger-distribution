use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OdRow {
    pub index: i32,
    pub from_station_code: String,
    pub to_station_code: String,
    pub count: i64,
    pub stations: Vec<String>,
}
