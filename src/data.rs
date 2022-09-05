// This file contains functions to generate data that will be used as input for the neural network
use crate::gaussian::Gaussian;

pub struct XnorDataset {
    x1: Vec<f32>,
    x2: Vec<f32>,
    y: Vec<bool>,
}

impl XnorDataset {
    pub fn new(num_samples: u32) -> XnorDataset {
        let mut g_1: Gaussian = Gaussian::new(0., 0.2);
        let mut g_2: Gaussian = Gaussian::new(1., 0.2);

        let mut x1: Vec<f32> = Vec::new();
        let mut x2: Vec<f32> = Vec::new();
        let mut y: Vec<bool> = Vec::new();

        x1.append(&mut g_1.samples(num_samples/4));
        x2.append(&mut g_1.samples(num_samples/4));
        y.append(&mut vec![true; (num_samples/4) as usize]);

        x1.append(&mut g_1.samples(num_samples/4));
        x2.append(&mut g_2.samples(num_samples/4));
        y.append(&mut vec![false; (num_samples/4) as usize]);

        x1.append(&mut g_2.samples(num_samples/4));
        x2.append(&mut g_1.samples(num_samples/4));
        y.append(&mut vec![false; (num_samples/4) as usize]);

        x1.append(&mut g_2.samples(num_samples/4));
        x2.append(&mut g_2.samples(num_samples/4));
        y.append(&mut vec![true; (num_samples/4) as usize]);

        XnorDataset { x1: x1, x2: x2, y: y } 
    }
}
