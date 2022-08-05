use rand::distributions::Uniform;
use rand::Rng;
use rand_distr::Beta;
use rand_distr::BetaError;
use rand_distr::Distribution;
use std::f64::consts::E;
use std::f64::consts::PI;

/// Reformulation of the beta distribution in terms of mean and standard deviation
/// Originally it accepts two shapes, alpha and beta
pub fn beta(mean: f64, stdev: f64) -> Result<Beta<f64>, BetaError> {
    let n = (mean * (1.0 - mean)) / stdev.powi(2);
    let alpha = mean * n;
    let beta = (1.0 - mean) * n;
    Beta::new(alpha, beta)
}

pub fn folded_normal(mean: f64, stdev: f64, x: f64) -> f64 {
    let common = 1.0 / (stdev * (2.0 * PI).powf(0.5));
    let positive = E.powf(-0.5 * ((x + mean) / stdev).powi(2));
    let negative = E.powf(-0.5 * ((x - mean) / stdev).powi(2));
    common * (positive + negative)
}

pub struct FoldedNormal {
    mean: f64,
    stdev: f64,
}

impl FoldedNormal {
    pub fn new(mean: f64, stdev: f64) -> FoldedNormal {
        FoldedNormal { mean, stdev }
    }
}

impl Distribution<f64> for FoldedNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        folded_normal(self.mean, self.stdev, rng.sample(Uniform::new(0.0, 1.0)))
    }
}

pub fn standardize_between(max: f64, min: f64, xs: Vec<f64>) -> Vec<f64> {
    xs.iter().map(|x| (x - min) / (max - min) * 100.0).collect()
}

pub fn clamp(x: f64) -> f64 {
    if x >= 0.0 && x <= 100.0 {
        x
    } else if x > 100.0 {
        100.0
    } else {
        0.0
    }
}
