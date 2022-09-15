mod gaussian;
mod data;
mod loss;
mod activation;
mod vectors;
mod neuralnetwork;

use neuralnetwork::NeuralNetwork;
use vectors::models::Vector2D;

fn main() {
    let x = Vector2D::new(vec![1., 2., 1., 1., 2., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.], [5, 3]);
    let y = Vector2D::new(vec![1., 0., 1., 1., 0.,], [5, 1]);
    let mut nn = NeuralNetwork::new(vec![3, 5, 6, 100, 1000, 32, 5, 1]);
    let h = nn.forward(x);
    nn.backward(y);
}