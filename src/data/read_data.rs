use crate::data::utils::*;
use crate::types::StationStairs;
use csv::StringRecord;
use std::cmp::Ordering;
use std::iter;
use std::ops::Deref;
use svg::parser::Event;

pub fn read_stair_locations(
    path: String,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let events = svg::open(path, &mut content)?;

    let mut guideline_pos: Vec<f64> = vec![];
    let mut start = None;
    let mut end = None;
    for event in events {
        if let Event::Tag(path, _, attrs) = event {
            if path == "sodipodi:guide" {
                let raw_pos = attrs.get("position").unwrap();
                let pos = raw_pos.deref();
                // to_string() turn &str into String, which create
                // a new allocation and own the data, because the &str pointer
                // would be dropped at the end
                let xpos_str = pos.split(',').next().unwrap().to_string();
                let xpos: f64 = xpos_str.parse().unwrap();

                let mut is_stair = true;
                if let Some(raw_label) = attrs.get("inkscape:label") {
                    let label = raw_label.deref();
                    if label == "start" {
                        start = Some(xpos);
                        is_stair = false;
                    } else if label == "end" {
                        end = Some(xpos);
                        is_stair = false;
                    };
                }

                if !xpos.is_nan() && is_stair {
                    guideline_pos.push(xpos);
                };
            };
        };
    }
    let start = start.unwrap();
    let end = end.unwrap();

    guideline_pos.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // to prevent stupid mistakes in labelling
    if start > end {
        // if start > end then start is max and end is min
        Ok(standardize_between(start, end, guideline_pos))
    } else {
        // if end > start then end is max and start is min
        Ok(standardize_between(end, start, guideline_pos))
    }
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

/// returns a mapping from lines (String) to stations and their data (Vec<StringRecord>)
pub fn read_link_load_data() -> Vec<(String, Vec<StringRecord>)> {
    let mut rdr = csv::Reader::from_path("data/001178992.csv").unwrap();
    let mut records = vec![];
    for result in rdr.records() {
        let record = result;
        if let Ok(r) = record {
            records.push(r);
        }
    }

    // add an empty row in the front so that partitioning works
    let first_line = StringRecord::from(
        iter::repeat("").take(records[0].len()).collect::<Vec<_>>(),
    );
    let mut r = vec![first_line];
    r.extend(records);
    let records = r;

    // partition line rows and station rows (and remove total rows)
    let mut result = vec![];
    for record in records {
        // if record is empty vec then exclude this row anyway
        let is_total_row =
            record.get(0).map(|string| string == "合計").unwrap_or(true);

        if record.iter().all(|string| string.is_empty()) {
            result.push(vec![]);
        } else if !is_total_row {
            let l = result.len() - 1;
            let last_vec = &mut result[l];
            last_vec.push(record);
        }
    }

    // turn results into (line, stations-in-this-line)
    result
        .iter()
        .map(|vec| {
            (
                vec[0].get(0).unwrap().to_string(),
                vec.iter().skip(1).cloned().collect::<Vec<_>>(),
            )
        })
        .collect()
}
