use statrs::distribution::Continuous;

/// Reformulation of the beta distribution in terms of mode and concentration
/// Larger concentration means more focused and lower variance
pub fn beta(mode: f64, concentration: f64, x: f64) -> f64 {
    assert!((0.0..1.0).contains(&mode));
    assert!(concentration >= 2.0);
    let alpha = mode * (concentration - 2.0) + 1.0;
    let beta = concentration - alpha;
    statrs::distribution::Beta::new(alpha, beta).unwrap().pdf(x)
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
