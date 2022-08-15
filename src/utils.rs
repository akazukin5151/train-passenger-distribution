pub fn sum_boarding_types<T>(
    boarding: &[(T, Vec<f64>, Vec<f64>, Vec<f64>)],
) -> Vec<f64> {
    boarding
        .iter()
        .fold(vec![], |mut acc, (_, far, close, uni)| {
            acc.extend(far);
            acc.extend(close);
            acc.extend(uni);
            acc
        })
}
