// This file contains loss functions that can be used by the neural network
use crate::vector;

pub fn cross_entropy_loss(h: &Vec<f32>, y: &Vec<f32>) -> f32 {
    // -(1/y.size) * ((y.T @ log(h)) + ((1 - y.T) @ log(1 - h)))

    (
        vector::dot(&y, &vector::logarithm(&h)) + 
        vector::dot(
        &vector::scalar_add(&vector::negate(&y), &1.),
        &vector::logarithm(
            &vector::scalar_add(&vector::negate(&h), &1.)
            )
        )
    ) * (-1. / y.len() as f32)
}

pub fn cross_entropy_derivative(h: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    // -(y // h) + ((1 - y) // (1 - h))

    vector::vector_add(
        &vector::negate(&vector::vector_divide(&y, &h)),
        &vector::vector_divide(
            &vector::scalar_add(&vector::negate(&y), &1.),
            &vector::scalar_add(&vector::negate(&h), &1.)
        )
    )
}


#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use super::*;

    #[test]
    fn test_cross_entropy_loss() {
        let mut h: Vec<f32> = vec![0., 0., 0.1, 0.1, 0.25, 0.25, 0.5, 0.5, 0.65, 0.65, 0.8, 0.8, 1., 1.];
        let mut y: Vec<f32> = vec![0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.];
        assert!(cross_entropy_loss(&h, &y) == INFINITY as f32);

        h[1] = 0.1;
        assert!(cross_entropy_loss(&h, &y) == INFINITY as f32);

        h[12] = 0.9;
        assert!(cross_entropy_loss(&h, &y) <= 0.9562 as f32);
        assert!(cross_entropy_loss(&h, &y) >= 0.9561 as f32);
    }

    #[test]
    fn test_cross_entropy_loss_derivative() {
        let mut h: Vec<f32> = vec![0., 1.];
        let mut y: Vec<f32> = vec![0., 1.];
        let mut d: Vec<f32> = cross_entropy_derivative(&h, &y);
        for e in d {
            assert!(e.is_nan());
        }

        let mut h: Vec<f32> = vec![0.1, 0.1];
        let mut y: Vec<f32> = vec![1., 0.];
        let mut d: Vec<f32> = cross_entropy_derivative(&h, &y);
        assert!(d[0] >= -10.001);
        assert!(d[0] <= -9.999);
        assert!(d[1] >= 1.111);
        assert!(d[1] <= 1.112);

        let mut h: Vec<f32> = vec![0.8, 0.8];
        let mut y: Vec<f32> = vec![1., 0.];
        let mut d: Vec<f32> = cross_entropy_derivative(&h, &y);
        assert!(d[0] >= -1.251);
        assert!(d[0] <= -1.249);
        assert!(d[1] <= 5.001);
        assert!(d[1] >= 4.999);
    }
}