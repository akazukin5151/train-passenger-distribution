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
    // mapping from every station to vector of
    // [number of people travelling from said station to every future station]
    let xs: Vec<Vec<i64>> = pairs
        .iter()
        .map(|(from, dests)| {
            dests
                .iter()
                .filter_map(|to| {
                    od_pairs.iter().find(|row| {
                        (row.stations.len() == 1 && row.stations[0] == "[]")
                            && ((row.from_station_code == ***from
                                && row.to_station_code == ***to)
                                // TODO
                                || (row.to_station_code == ***from
                                    && row.from_station_code == ***to))
                    })
                })
                .map(|x| x.count)
                .collect()
        })
        .collect();

    assert!(xs.len() == stations.len());

    // the length of the inner vectors decrease by one after every station
    // the first station has n_stations - 1 items, because no passengers are travelling
    // from the first to the first; only from first to second and so on.
    // thus the first station's vector excludes itself, the second stations'
    // excludes the first station (which the train has already passed)
    // plus itself.
    assert!(
        xs.iter().map(|ys| ys.len()).collect::<Vec<_>>()
            == (0..stations.len()).rev().collect::<Vec<_>>()
    );

    // the len is stations.len() - 1 because first station's inner vec excludes itself
    let mut n_alighters_at_every_station: Vec<i64> = vec![0];
    n_alighters_at_every_station.extend(transpose_sum(xs[0].len(), &xs));

    assert!(n_alighters_at_every_station.len() == stations.len());

    n_alighters_at_every_station
}

pub fn transpose_sum(max_i: usize, xs: &[Vec<i64>]) -> Vec<i64> {
    let x: Vec<Vec<&i64>> = xs
        .iter()
        .map(|vec| {
            let mut v: Vec<_> = vec.iter().rev().collect();
            while v.len() < max_i {
                v.push(&0)
            }
            v
        })
        .collect();

    (0..max_i)
        .map(|idx| {
            let mut res = 0_i64;
            for item in &x {
                res += item[idx];
            }
            res
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    fn input() -> Vec<Vec<i64>> {
        vec![
            vec![1, 5, 2, 7],
               vec![9, 3, 7],
                  vec![6, 2],
                     vec![7]
        ]
    }

    #[test]
    fn test_transpose_sum() {
        let xs = input();
        let res = transpose_sum(xs[0].len(), &xs);
        assert_eq!(res, vec![1, 14, 11, 23])
    }
}
