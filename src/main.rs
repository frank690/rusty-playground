mod data;
mod gaussian;

fn main() {
    data::generate(None);
    let mut g = gaussian::Gaussian::new(0., 1.);
    println!("{}", g.info());

    println!("random gaussian sample: {}", g.sample());
    println!("pdf: {}", g.pdf(-100.));
}