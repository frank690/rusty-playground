mod vector;
mod gaussian;
mod data;
mod loss;
mod activation;

fn main() {
    println!("{}", f32::ln(1.));
    let mut y: Vec<f32> = vec![3., 2.2, -5.];
    let mut h: Vec<f32> = vec![2., -0.5, -1.25];
    let mut r = loss::cross_entropy_loss(&h, &y);
    println!("r: {}", &r);

    let mut d: Vec<f32> = loss::cross_entropy_derivative(&h, &y);
    println!("d: {:?}", &d);

    let mut m: f32 = -0.;
    println!("exp(-{}): {}", &m, (-m).exp());
    println!("sigmoid: {}", activation::sigmoid(&-0.1));
}