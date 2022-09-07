// This file contains loss functions that can be used by the neural network
use crate::vector;

pub fn cross_entropy_loss(h: Vec<f32>, y: Vec<bool>) -> f32 {
    // -(1/y.size) * ((y.T @ np.log(h)) + ((1 - y.T) @ np.log(1 - h)))
    let a: f32 = 0.123;
    let s: f32 = -1. / y.len() as f32;
    vector::scalar_multiply(
        vector::logarithm(&mut h),
        &s
    )
    a
}