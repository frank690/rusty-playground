// This file contains all neural network implementation related functions.

use crate::{vectors::models::Vector2D, gaussian::Gaussian, activation::{self, sigmoid_derivative}, loss};

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

pub struct NeuralNetworkGradients {
    pub layers: usize,
    pub z: Vec<Vector2D>,
    pub a: Vec<Vector2D>,
    pub dz: Vec<Vector2D>,
    pub da: Vec<Vector2D>,
}

impl NeuralNetworkGradients {
    pub fn new(layers: usize) -> NeuralNetworkGradients {
        NeuralNetworkGradients { 
            layers, 
            z: vec![Vector2D::default(); layers], 
            a: vec![Vector2D::default(); layers], 
            dz: vec![Vector2D::default(); layers], 
            da: vec![Vector2D::default(); layers]
        }
    }

    pub fn h(&self) -> Vector2D {
        self.a[self.a.len()-1].clone()
    }
}

pub struct NeuralNetwork {
    pub shape: Vec<usize>,
    pub weights: Vec<Vector2D>,
    pub biases: Vec<f32>,
    learning_rate: f32,
    internal: NeuralNetworkGradients,
}

impl NeuralNetwork {
    pub fn new(shape: Vec<usize>) -> NeuralNetwork {
        let weights = initialize_weights(&shape);
        let biases = initialize_biases(&shape);
        let internal = NeuralNetworkGradients::new(shape.len()-1);
        let learning_rate = 1.;
        NeuralNetwork { shape, weights, biases, internal, learning_rate }
    }

    pub fn forward(&mut self, input: Vector2D) -> Vector2D {
        self.internal.a[0] = input;

        for layer in 0..self.internal.layers-1 {
            self.internal.z[layer] = &self.internal.a[layer].dot(&self.weights[layer]) + &self.biases[layer];
            self.internal.a[layer+1] = activation::sigmoid(&self.internal.z[layer]);
        }
        println!("result values: {:?} with shape: {:?}", self.internal.h().values, self.internal.h().shape);
        return self.internal.h()
    }

    pub fn backward(&mut self, true_output: Vector2D) {
        self.internal.da[self.internal.layers-1] = loss::cross_entropy_derivative(self.internal.h(), true_output);
        for layer in self.internal.layers-1..-1 {
            self.internal.dz[layer] = self.internal.da[layer] * sigmoid_derivative(&self.internal.z[layer]);
        }
    }
}