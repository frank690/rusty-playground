// This file holds functions to sample from an arbitrary gaussian distribution
use rand::{Rng, rngs::ThreadRng, thread_rng};

pub struct Gaussian {
    mean: f64,
    variance: f64,
    random_number_generator: ThreadRng
}

impl Gaussian {
    pub fn new(mean: f64, variance: f64) -> Gaussian {
        Gaussian { mean: mean, variance: variance, random_number_generator: thread_rng() }
    }

    pub fn info(&self) -> String {
        format!("Gaussian distribution with mean of {} and variance of {}.", self.mean, self.variance)
    }

    pub fn sample(&mut self) -> f64 {
        self.random_number_generator.gen::<f64>()
    }

    pub fn pdf(&self, x: f64) -> f64 {
        1./f64::sqrt(2.* std::f64::consts::PI * self.variance) * 
        f64::exp(-f64::powi(x - self.mean, 2) / (2. * self.variance))
    }
}
