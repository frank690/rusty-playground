mod gaussian;
mod data;
mod loss;
mod activation;
mod vectors;
mod neuralnetwork;

use loss::cross_entropy_loss;
use neuralnetwork::NeuralNetwork;
use vectors::models::Vector2D;

fn main() {
    let x = Vector2D::new(vec![1., 2., 1., 1., 2., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.], [5, 3]);
    let nn = NeuralNetwork::new(vec![3, 5, 6, 100, 1000, 32, 5, 1]);
    let h = nn.forward(x);

    let h: Vector2D = Vector2D::new(
        vec![0., 0.1, 0.1, 0.1, 0.25, 0.25, 0.5, 0.5, 0.65, 0.65, 0.8, 0.8, 0.9, 1.],
        [14, 1]
    );

    let y: Vector2D = Vector2D::new(
        vec![0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.],
        [14, 1]
    );

    let z = cross_entropy_loss(h, y);
    println!("CE loss values: {:?} with shape: {:?}", z.values, z.shape);
}