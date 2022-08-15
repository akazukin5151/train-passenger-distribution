use std::f64::consts::E;
use std::f64::consts::PI;

pub fn scotts(n: f64) -> f64 {
    n.powf(-1.0 / (1.0 + 4.0))
}

fn gaussian(u: f64) -> f64 {
    1.0 / ((2.0 * PI).powi(1 / 2)) * E.powf((-1.0 / 2.0) * u.powi(2))
}

pub fn kernel_density_estimator(xs: &[f64], bandwidth: f64, x: f64) -> f64 {
    let n = xs.len() as f64;
    let summed: f64 = xs.iter().map(|xi| gaussian((x - xi) / bandwidth)).sum();
    (1.0 / (n * bandwidth)) * summed
}

pub fn make_kde(multiplier: f64, tp: &[f64]) -> Vec<(f64, f64)> {
    (0..=100)
        .map(|num| {
            let x = num as f64;
            let bandwidth = scotts(tp.len() as f64) * multiplier;
            let y = kernel_density_estimator(tp, bandwidth, x);
            (x, y)
        })
        .collect()
}

