use crate::data::utils::*;
use crate::types::*;
use std::cmp::Ordering;
use std::ops::Deref;
use svg::parser::Event;

pub fn read_data_from_file(
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

pub fn read_od_row() -> Vec<OdRow> {
    let mut rdr = csv::Reader::from_path("data/out.csv").unwrap();
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Result<OdRow, _> = result;
        if let Ok(r) = record {
            records.push(r);
        }
    }
    records
}
