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
        &vector::vector_divide(&y, &h),
        &vector::vector_divide(
            &vector::scalar_add(&vector::negate(&y), &1.),
            &vector::scalar_add(&vector::negate(&h), &1.)
        )
    )
}