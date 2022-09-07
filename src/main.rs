mod vector;
mod gaussian;
mod data;
//mod loss;

fn main() {
    let mut v: Vec<f32> = vec![1.2, 3.2, 0.0001];
    let s: f32 = 3.141;

    println!("v: {:?}", &v);
    println!("s: {:?}", &s);
    let mut ln_v = vector::logarithm(&v);
    println!("ln_v: {:?}", &ln_v);
    println!("v: {:?}", &v);
    println!("s: {:?}", &s);

    let mut scmu = vector::scalar_multiply(&v, &s);
    println!("scmu: {:?}", &scmu);
    println!("v: {:?}", &v);
    println!("s: {:?}", &s);

}