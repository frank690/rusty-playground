// This file contains all neural network implementation related functions.

use crate::{vectors::models::Vector2D, gaussian::Gaussian, activation::{self, sigmoid_derivative}, loss};


fn initialize_weights(shape: &Vec<usize>) -> Vec<Vector2D> {
    let mut g: Gaussian = Gaussian::new(0., 1.);
    let mut weights: Vec<Vector2D> = vec![];
    for idx in 0..shape.len() - 1 {
        let weight_shape: [usize; 2] = [shape[idx], shape[idx+1]];
        let weight_values: Vec<f64> = g.samples(shape[idx] * shape[idx+1]);
        let mut weight: Vector2D = Vector2D::new(weight_values, weight_shape);
        weight = 2. * weight - 1.;
        weights.push(weight);
    }
    return weights;
}

fn initialize_biases(shape: &Vec<usize>) -> Vec<Vector2D> {
    let mut biases: Vec<Vector2D> = vec![];
    for idx in 0..shape.len() - 1 {
        let bias_shape: [usize; 2] = [1, shape[idx+1]];
        let bias_values: Vec<f64> = vec![0.; shape[idx+1]];
        let bias: Vector2D = Vector2D::new(bias_values, bias_shape);
        biases.push(bias);
    }
    return biases;
}

pub struct Parameters {
    pub weights: Vec<Vector2D>,
    pub biases: Vec<Vector2D>,
    pub z: Vec<Vector2D>,
    pub a: Vec<Vector2D>
}

impl Parameters {
    pub fn new(layers: usize, weights: Vec<Vector2D>, biases: Vec<Vector2D>) -> Parameters {
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
    pub learning_rate: f64,
    pub layers: usize
}

impl HyperParameters {
    pub fn new(shape: Vec<usize>, learning_rate: f64) -> HyperParameters {
        let layers: usize = shape.len();
        HyperParameters { shape, learning_rate, layers }
    }
}

pub struct Gradients {
    pub z: Vec<Vector2D>,
    pub a: Vec<Vector2D>,
    pub weights: Vec<Vector2D>,
    pub biases: Vec<Vector2D>
}

impl Gradients {
    pub fn new(layers: usize) -> Gradients {
        Gradients { 
            z: vec![Vector2D::default(); layers-1], 
            a: vec![Vector2D::default(); layers],
            weights: vec![Vector2D::default(); layers-1],
            biases: vec![Vector2D::default(); layers-1],
        }
    }
}

pub struct NeuralNetwork {
    pub parameters: Parameters,
    pub hyperparameters: HyperParameters,
    pub gradients: Gradients,
}

impl NeuralNetwork {
    pub fn new(shape: Vec<usize>) -> NeuralNetwork {
        let learning_rate: f64 = 1.;
        let hyperparameters: HyperParameters = HyperParameters::new(shape, learning_rate);

        let weights = initialize_weights(&hyperparameters.shape);
        let biases = initialize_biases(&hyperparameters.shape);
        let parameters: Parameters = Parameters::new(hyperparameters.layers, weights, biases);

        let gradients = Gradients::new(hyperparameters.layers);

        NeuralNetwork { parameters, gradients, hyperparameters }
    }

    pub fn forward(&mut self, input: &Vector2D) -> Vector2D {
        self.parameters.a[0] = input.clone();
        for layer in 0..self.hyperparameters.layers-1 {
            self.parameters.z[layer] = self.parameters.a[layer].dot(&self.parameters.weights[layer]).row_add(&self.parameters.biases[layer]);
            self.parameters.a[layer+1] = activation::sigmoid(&self.parameters.z[layer]);
        }
        return self.parameters.h()
    }

    pub fn backward(&mut self, true_output: &Vector2D) {
        self.gradients.a[self.hyperparameters.layers-1] = loss::cross_entropy_derivative(self.parameters.h(), true_output);
        for layer in (0..self.hyperparameters.layers-1).rev() {
            self.gradients.z[layer] = &self.gradients.a[layer+1] * sigmoid_derivative(&self.parameters.z[layer]);
            self.gradients.a[layer] = self.gradients.z[layer].dot(&self.parameters.weights[layer].transpose());
            
            self.gradients.biases[layer] = self.gradients.z[layer].mean(0);
            self.gradients.weights[layer] = self.parameters.a[layer].transpose().dot(&self.gradients.z[layer]) / self.parameters.h().len() as f64;
        }
    }

    pub fn update(&mut self) {
        for layer in 0..self.hyperparameters.layers-1 {
            self.parameters.weights[layer] = &self.parameters.weights[layer] - self.hyperparameters.learning_rate * &self.gradients.weights[layer];
            self.parameters.biases[layer] = &self.parameters.biases[layer] - self.hyperparameters.learning_rate * &self.gradients.biases[layer];
        }
    }

    pub fn training(&mut self, input: Vector2D, true_output: Vector2D, epochs: usize, verbose: bool) {
        for epoch in 0..epochs {
            let h: Vector2D = self.forward(&input);
            let loss: Vector2D = loss::cross_entropy_loss(&h, &true_output);

            if verbose {
                println!("Epoch {}: {}", epoch, loss.get_value(0));
            }

            self.backward(&true_output);
            self.update();
        }
    }
}