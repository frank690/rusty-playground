// This file contains functions to generate data that will be used as input for the neural network
use crate::{gaussian::Gaussian, vectors::models::Vector2D};

pub struct XnorDataset {
    pub x: Vector2D,
    pub y: Vector2D,
}

impl XnorDataset {
    pub fn new(num_samples: usize) -> XnorDataset {
        let mut g_1: Gaussian = Gaussian::new(0., 0.2);
        let mut g_2: Gaussian = Gaussian::new(1., 0.2);

        let mut x_values: Vec<f64> = Vec::new();
        let mut y_values: Vec<f64> = Vec::new();

        x_values.append(&mut g_1.samples(num_samples/2));
        x_values.append(&mut g_2.samples(num_samples/2));
        x_values.append(&mut g_1.samples(num_samples/4));
        x_values.append(&mut g_2.samples(num_samples/4));
        x_values.append(&mut g_1.samples(num_samples/4));
        x_values.append(&mut g_2.samples(num_samples/4));

        y_values.append(&mut vec![1.; num_samples/4]);
        y_values.append(&mut vec![0.; num_samples/2]);
        y_values.append(&mut vec![1.; num_samples/4]);

        let x: Vector2D = Vector2D::new(x_values, [2, num_samples]).transpose();
        let y: Vector2D = Vector2D::new(y_values, [num_samples, 1]);

        XnorDataset { x, y } 
    }

    pub fn print(&self, num_samples: usize) {
        println!("x= ");
        for idx in 0..num_samples {
            println!("[{}, {}],", &self.x[2*idx], &self.x[2*idx+1],);
        }

        println!("y= ");
        for idx in 0..num_samples {
            println!("[{}],", &self.y[idx],);
        }
    }
}
