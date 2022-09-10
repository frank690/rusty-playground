mod vector;
mod gaussian;
mod data;
mod loss;
mod activation;
mod vectors;
mod neuralnetwork;

use neuralnetwork::NeuralNetwork;
use vectors::models::Vector2D;

fn main() {
    let mut v1: Vector2D = Vector2D::new(vec![1., 2., 3., 4., 5., 6.], [3, 2]);
    let mut v2: Vector2D = Vector2D::new(vec![1., 0., 1., 0., 1., 0.], [2, 3]);    

    println!("v1: {:?}", &v1.values);
    println!("v2: {:?}", &v2.values);

    let result = v1.dot(&v2);

    println!("v1 * v2: {:?} with shape {:?}", &result.values, &result.shape);

    let x = Vector2D::new(vec![1., 2., 1., 1., 2., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.], [5, 3]);
    let nn = NeuralNetwork::new(vec![3, 5, 6, 1]);
    nn.forward(x);
}