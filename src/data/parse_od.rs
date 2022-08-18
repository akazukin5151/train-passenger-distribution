use crate::types::OdRow;

// TODO: lots of zeros in the data. Look at script and find out what happened to
// pairs that are only flipped. Because direction is important this time
// A -> B is needed and not B -> A, but current dataset has eliminated either
// pair.
// Bring back the separation
/// calculates the cumulative number of boarders at every given station
pub fn parse_od(od_pairs: &[OdRow], stations: &[&str]) -> Vec<i64> {
    // from every station to every future station in the line
    let pairs: Vec<(&&str, Vec<&&str>)> = stations
        .iter()
        .enumerate()
        .map(|(idx, from)| (from, stations.iter().skip(idx + 1).collect()))
        .collect();

    // mapping from every station to vector of future stations in the line
    // but extracted the number of people, turning it into
    // mapping from every station to vector of [people travelling from said station to every
    // future station]
    let xs: Vec<Vec<i64>> = pairs
        .iter()
        .map(|(from, dests)| {
            dests
                .iter()
                .filter_map(|to| {
                    od_pairs.iter().find(|row| {
                        (row.stations.len() == 1 && row.stations[0] == "[]")
                            && (row.from_station_code == ***from
                                && row.to_station_code == ***to)
                    })
                })
                .map(|x| x.count)
                .collect()
        })
        .collect();

    assert!(xs.len() == stations.len());

    // total number of people at every line section
    // xs[0] = number of people going from 1st station to 2nd station + 1st to 3rd, ...
    // xs[1] = number of people going from 2nd station to 3rd station + 2nd to 4th, ...
    let xs: Vec<i64> = xs.iter().map(|vec| vec.iter().sum()).collect();

    assert!(xs.len() == stations.len());

    xs
}
