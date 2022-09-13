// This file contains activation functions

use crate::vectors::models::Vector2D;

fn solo_sigmoid(v: &f32) -> f32 {
    1. / (1. + (-v).exp())
}

pub fn sigmoid(v: &Vector2D) -> Vector2D {
    let mut new_values = vec![];
    for value in &v.values {
        new_values.push(solo_sigmoid(&value));
    }
    Vector2D::new(new_values, v.shape)
}

pub fn sigmoid_derivative(v: &Vector2D) -> Vector2D {
    let sig = sigmoid(v);
    let sag = 1. as f32 - &sig;
    sig * sag
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_sigmoid() {
        assert!(solo_sigmoid(&0.) == 0.5);
        assert!(solo_sigmoid(&-0.) == 0.5);
        assert!(solo_sigmoid(&0.00001) > 0.5);
        assert!(solo_sigmoid(&-0.00001) < 0.5);     
    }
}