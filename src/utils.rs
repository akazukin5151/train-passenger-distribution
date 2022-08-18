use crate::types::BoardingData;

pub fn sum_boarding_types(boarding: &Vec<BoardingData>) -> Vec<f64> {
    boarding.iter().cloned().fold(vec![], |mut acc, bd| {
        acc.extend(bd.beta_far);
        acc.extend(bd.beta_close);
        acc.extend(bd.uniform);
        acc
    })
}
