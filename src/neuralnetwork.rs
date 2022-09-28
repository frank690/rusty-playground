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

pub struct Parameters {
    pub weights: Vec<Vector2D>,
    pub biases: Vec<f32>,
    pub z: Vec<Vector2D>,
    pub a: Vec<Vector2D>
}

impl Parameters {
    pub fn new(layers: usize, weights: Vec<Vector2D>, biases: Vec<f32>) -> Parameters {
        Parameters { 
            weights, 
            biases,
            z: vec![Vector2D::default(); layers], 
            a: vec![Vector2D::default(); layers],
        }
    }

    pub fn h(&self) -> Vector2D {
        self.a[self.a.len()-1].clone()
    }
}

pub struct HyperParameters {
    pub shape: Vec<usize>,
    pub learning_rate: f32,
    pub layers: usize
}

impl HyperParameters {
    pub fn new(shape: Vec<usize>, learning_rate: f32) -> HyperParameters {
        let layers: usize = shape.len() - 1;
        HyperParameters { shape, learning_rate, layers }
    }
}

pub struct Gradients {
    pub z: Vec<Vector2D>,
    pub a: Vec<Vector2D>,
    pub weights: Vec<Vector2D>,
    pub biases: Vec<f32>
}

impl Gradients {
    pub fn new(layers: usize) -> Gradients {
        Gradients { 
            z: vec![Vector2D::default(); layers], 
            a: vec![Vector2D::default(); layers],
            weights: vec![Vector2D::default(); layers],
            biases: vec![0.; layers],
        }
    }
}

pub struct NeuralNetwork {
    parameters: Parameters,
    hyperparameters: HyperParameters,
    gradients: Gradients,
}

impl NeuralNetwork {
    pub fn new(shape: Vec<usize>) -> NeuralNetwork {
        let learning_rate: f32 = 1.;
        let hyperparameters: HyperParameters = HyperParameters::new(shape, learning_rate);

        let weights = initialize_weights(&hyperparameters.shape);
        let biases = initialize_biases(&hyperparameters.shape);
        let parameters: Parameters = Parameters::new(hyperparameters.layers, weights, biases);

        let gradients = Gradients::new(hyperparameters.layers);

        NeuralNetwork { parameters, gradients, hyperparameters }
    }

    pub fn forward(&mut self, input: Vector2D) -> Vector2D {
        self.parameters.a[0] = input;
        println!("input shape a[0]={:?}", self.parameters.a[0].shape);
        for layer in 0..self.hyperparameters.layers-1 {
            self.parameters.z[layer] = &self.parameters.a[layer].dot(&self.parameters.weights[layer]) + &self.parameters.biases[layer];
            self.parameters.a[layer+1] = activation::sigmoid(&self.parameters.z[layer]);

            println!("layer {}: z[{}].shape={:?}, a[{}+1].shape={:?}", &layer, &layer, &self.parameters.z[layer].shape, layer, &self.parameters.a[layer+1].shape);
        }
        println!("result values: {:?} with shape: {:?}", self.parameters.h().values, self.parameters.h().shape);
        return self.parameters.h()
    }

    pub fn backward(&mut self, true_output: Vector2D) {
        self.gradients.a[self.hyperparameters.layers-1] = loss::cross_entropy_derivative(self.parameters.h(), true_output);
        for layer in (0..self.hyperparameters.layers).rev() {
            self.gradients.z[layer] = &self.gradients.a[layer] * sigmoid_derivative(&self.parameters.z[layer]);
            self.gradients.a[layer-1] = self.gradients.z[layer].dot(&self.parameters.weights[layer].transpose());

            self.gradients.biases[layer] = self.gradients.z[layer].overall_mean();
            self.gradients.weights[layer] = self.parameters.a[layer-1].transpose().dot(&self.gradients.z[layer]) / self.parameters.h().len() as f32;
        }
    }
}