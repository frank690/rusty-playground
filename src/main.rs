mod data;
mod gaussian;

fn main() {
    data::generate(None);
    let mut g = gaussian::Gaussian::new(1.23, 0.42);
    println!("{}", g.info());

    println!("random gaussian sample: {}", g.sample());
}

