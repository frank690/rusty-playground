// This file holds functions to sample from an arbitrary gaussian distribution
use rand::{Rng, rngs::ThreadRng, thread_rng};

pub struct Gaussian {
    mean: f64,
    std: f64,
    random_number_generator: ThreadRng
}

impl Gaussian {
    pub fn new(mean: f64, std: f64) -> Gaussian {
        Gaussian { mean: mean, std: std, random_number_generator: thread_rng() }
    }

    fn box_muller(&mut self) -> Vec<f64> {
        let x_1: f64 = self.random_number_generator.gen::<f64>();
        let x_2: f64 = self.random_number_generator.gen::<f64>();

        let mut g: Vec<f64> = Vec::new();
        
        let y: f64 = f64::sqrt(-2. * f64::ln(x_1)) * f64::cos(2. * std::f64::consts::PI * x_2);
        let z: f64 = f64::sqrt(-2. * f64::ln(x_1)) * f64::sin(2. * std::f64::consts::PI * x_2);
        g.push(self.mean + self.std * y);
        g.push(self.mean + self.std * z);
        g
    }

    pub fn samples(&mut self, n: usize) -> Vec<f64> {
        let mut numbers: Vec<f64> = Vec::new();

        for _ in 0..(n/2) {
            numbers.append(&mut self.box_muller());
        }

        if (n%2) == 1 {
            numbers.append(&mut vec![self.box_muller().remove(0)]);
        }


        numbers
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_mean_and_var() {
        let mut rng = thread_rng();
        let n: usize = 54321;
        for _ in 0..10 {
            let random_mean: f64 = 5. * rng.gen::<f64>();
            let random_std: f64 = 5. * rng.gen::<f64>();
            let mut g: Gaussian = Gaussian::new(random_mean, random_std);
            let s: Vec<f64> = g.samples(n);

            assert!(mean(&s) <= random_mean * 1.1);
            assert!(mean(&s) >= random_mean * 0.9);
            assert!(standard_deviation(&s) <= random_std * 1.1);
            assert!(standard_deviation(&s) >= random_std * 0.9);
            assert!(s.len() as usize == n);
        }
    }

    fn mean(numbers: &Vec<f64>) -> f64 {
        let sum: f64 = numbers.iter().sum();
        sum / numbers.len() as f64
    }

    fn standard_deviation(numbers: &Vec<f64>) -> f64 {
        let m: f64 = mean(numbers);
        let mut v: Vec<f64> = Vec::new();
        for number in numbers {
            v.push(f64::powi(number - m, 2));
        }

        f64::sqrt(v.iter().sum::<f64>() / (v.len() - 1) as f64)
    }
}