mod vector;
mod gaussian;
mod data;
mod loss;
mod activation;
mod vectors;

use vectors::models::Vector2D;

fn main() {
    let v: f32 = 1.5;
    let values: Vec<f32> = vec![0., 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7];
    let shape: [usize; 2] = [4, 2];

    let v2d: Vector2D = Vector2D { values, shape };
    println!("{}", v2d[5]);
    println!("{}", v2d[(3, 1)]);
}