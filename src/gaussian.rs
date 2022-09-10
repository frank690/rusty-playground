// This file holds functions to sample from an arbitrary gaussian distribution
use rand::{Rng, rngs::ThreadRng, thread_rng};

pub struct Gaussian {
    mean: f32,
    variance: f32,
    std: f32,
    random_number_generator: ThreadRng
}

impl Gaussian {
    pub fn new(mean: f32, variance: f32) -> Gaussian {
        Gaussian { mean: mean, variance: variance, std: f32::sqrt(variance), random_number_generator: thread_rng() }
    }

    fn box_muller(&mut self) -> Vec<f32> {
        let x_1: f32 = self.random_number_generator.gen::<f32>();
        let x_2: f32 = self.random_number_generator.gen::<f32>();

        let mut g: Vec<f32> = Vec::new();
        
        let y: f32 = f32::sqrt(-2. * f32::ln(x_1)) * f32::cos(2. * std::f32::consts::PI * x_2);

        g.push(self.mean + self.std * y);
        g
    }

    pub fn samples(&mut self, n: usize) -> Vec<f32> {
        let mut numbers: Vec<f32> = Vec::new();

        for _ in 0..n {
            numbers.append(&mut self.box_muller());
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
        let n: usize = 100000;
        for _ in 0..10 {
            let random_mean: f32 = 5. * rng.gen::<f32>();
            let random_variance: f32 = 5. * rng.gen::<f32>();
            let mut g: Gaussian = Gaussian::new(random_mean, random_variance);
            let s: Vec<f32> = g.samples(n);

            assert!(mean(&s) <= random_mean * 1.2);
            assert!(mean(&s) >= random_mean * 0.8);
            assert!(variance(&s) <= random_variance * 1.2);
            assert!(variance(&s) >= random_variance * 0.8);
            assert!(s.len() as usize == n);
        }
    }

    fn mean(numbers: &Vec<f32>) -> f32 {
        let sum: f32 = numbers.iter().sum();
        sum / numbers.len() as f32
    }

    fn variance(numbers: &Vec<f32>) -> f32 {
        let m: f32 = mean(numbers);
        let mut v: Vec<f32> = Vec::new();
        for number in numbers {
            v.push(f32::powi(number - m, 2));
        }

        v.iter().sum::<f32>() / (v.len() - 1) as f32
    }
}