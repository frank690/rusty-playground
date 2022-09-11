// This file contains all neural network implementation related functions.

use crate::{vectors::models::Vector2D, gaussian::Gaussian, activation};

pub struct NeuralNetwork {
    pub shape: Vec<usize>,
    pub weights: Vec<Vector2D>,
    pub biases: Vec<f32>,
}

fn initialize_weights(shape: &Vec<usize>) -> Vec<Vector2D> {
    let mut g: Gaussian = Gaussian::new(0., 1.);
    let mut weights: Vec<Vector2D> = vec![];
    for idx in 0..shape.len()-1 {
        let weight_shape: [usize; 2] = [shape[idx], shape[idx+1]];
        let weight_values: Vec<f32> = g.samples(shape[idx] * shape[idx+1]);
        let weight: Vector2D = Vector2D::new(weight_values, weight_shape);
        weights.push(weight);
    }
    return weights;
}

fn initialize_biases(shape: &Vec<usize>) -> Vec<f32> {
    let mut biases: Vec<f32> = vec![];
    for _ in 0..shape.len()-1 {
        biases.push(0.);
    }
    return biases;
}

impl NeuralNetwork {
    pub fn new(shape: Vec<usize>) -> NeuralNetwork {
        let weights = initialize_weights(&shape);
        let biases = initialize_biases(&shape);
        NeuralNetwork { shape, weights, biases }
    }

    pub fn forward(&self, input: Vector2D) -> Vector2D {
        let mut a = input;

        for (weight, bias) in self.weights.iter().zip(&self.biases) {
            let z = a * weight + *bias;
            a = activation::sigmoid(&z);
        }
        println!("result values: {:?} with shape: {:?}", a.values, a.shape);
        return a
    }
}