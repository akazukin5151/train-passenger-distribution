use rand_distr::Beta;
use rand_distr::BetaError;
use statrs::function::gamma::gamma;

/// Reformulation of the beta distribution in terms of mean and standard deviation
/// Originally it accepts two shapes, alpha and beta
pub fn beta(mean: f64, stdev: f64) -> Result<Beta<f64>, BetaError> {
    let n = (mean * (1.0 - mean)) / stdev.powi(2);
    let alpha = mean * n;
    let beta = (1.0 - mean) * n;
    Beta::new(alpha, beta)
}

/// x must be between 0 and 1 inclusive
pub fn beta_(mean: f64, stdev: f64, x: f64) -> f64 {
    assert!(x >= 0.0);
    assert!(x <= 100.0);
    let n = (mean * (1.0 - mean)) / stdev.powi(2);
    let alpha = mean * n;
    let beta = (1.0 - mean) * n;
    let constant = gamma(alpha + beta) / (gamma(alpha) * gamma(beta));
    constant * x.powf(alpha - 1.0) * (1.0 - x).powf(beta - 1.0)
}

pub fn uniform(start: f64, end: f64, x: f64) -> f64 {
    if start <= x && x <= end {
        1.0 / (end - start)
    } else {
        0.0
    }
}

pub fn standardize_between(max: f64, min: f64, xs: Vec<f64>) -> Vec<f64> {
    xs.iter().map(|x| (x - min) / (max - min) * 100.0).collect()
}

pub fn clamp(x: f64) -> f64 {
    if x > 0.0 && x < 100.0 {
        x
    } else if x > 100.0 {
        99.9
    } else {
        0.01
    }
}
