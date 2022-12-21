// This file contains all implementation of custom vector models (structs)

#[derive(Clone)]
pub struct Vector2D {
    pub values: Vec<f64>,
    pub shape: [usize; 2]
}

impl Vector2D {
    pub fn new(values: Vec<f64>, shape: [usize; 2]) -> Vector2D {
        if values.len() != shape[0] * shape[1] {
            panic!("Shape {:?} does not match length {}", shape, values.len());
        }
        Vector2D { values, shape }
    }

    pub fn print(&self) {
        for row in 0..self.shape[0] {
            for column in 0..self.shape[1] {
                print!("{}, ", self.get_mat_value(row, column));
            }
            print!("\n");
        }
    }

    pub fn len(self) -> usize {
        self.values.len()
    }

    pub fn transpose(&self) -> Vector2D {
        let new_shape: [usize; 2] = [self.shape[1], self.shape[0]];
        let mut new_values: Vec<f64> = vec![];

        for row in 0..new_shape[0] {
            for column in 0..new_shape[1] {
                new_values.push(*self.get_mat_value(column, row));
            }
        }
        Vector2D::new(new_values, new_shape)    
    }

    pub fn row_add(&mut self, b_vector: &Vector2D) -> Vector2D {
        if self.shape[1] != b_vector.shape[1] {
            panic!("Can not row-wise add vector with shape {:?} to vector with shape {:?}", b_vector.shape, self.shape);
        } else {
            let mut result: Vec<f64> = vec![];
            let result_shape: [usize; 2] = self.shape.clone();
            
            for row in 0..result_shape[0] {
                result.append(&mut (self.get_mat_row_values(row) + b_vector.clone()).values);
            }
            Vector2D::new(result, result_shape)
        }
    }

    pub fn dot(&mut self, b_vector: &Vector2D) -> Vector2D {
        if self.shape[1] != b_vector.shape[0] {
            panic!("Can not dot multiply vectors with shape {:?} @ {:?}", self.shape, b_vector.shape);
        } else {
            let mut result: Vec<f64> = vec![];
            let result_shape: [usize; 2] = [self.shape[0], b_vector.shape[1]];

            for row in 0..result_shape[0] {
                for col in 0..result_shape[1] {
                    let mut value: f64 = 0.;
                    for idx in 0..self.shape[1] {
                        let a = self.get_mat_value(row, idx);
                        let b = b_vector.get_mat_value(idx, col);
                        if (*a == 0.) | (*b == 0.) {
                            continue;
                        }
                        value += a * b;
                    }
                    result.push(value);
                }
            }
            Vector2D::new(result, result_shape)
        }
    }

    pub fn ln(&self) -> Vector2D {
        let mut log_values = vec![];
        for v in &self.values {
            log_values.push(f64::ln(*v));
        }
        Vector2D::new(log_values, self.shape)
    }

    pub fn mean(&mut self, axis: usize) -> Vector2D {
        let mut new_shape: [usize; 2] = self.shape.clone();
        let mut new_values: Vec<f64> = vec![];

        if axis == 0 {
            for column in 0..self.shape[1] {
                let mut value: f64 = 0.;
                for row in 0..self.shape[0] {
                    value += self.get_mat_value(row, column);
                }
                new_values.push(value / self.shape[0] as f64);
            }
            new_shape[0] = 1;
        } else if axis == 1 {
            for row in 0..self.shape[0] {
                let mut value: f64 = 0.;
                for column in 0..self.shape[1] {
                    value += self.get_mat_value(row, column);
                }
                new_values.push(value / self.shape[1] as f64);
            }
            new_shape[1] = 1;
        } else {
            panic!("Vector2D only has 2 axis (0 or 1) to compute mean over. Given axis {} does not exist", axis);
        }
        Vector2D::new(new_values, new_shape)
    }

    pub fn overall_mean(&mut self) -> f64 {
        let mut mean_0: Vector2D = self.mean(0);
        let mean_01: Vector2D = mean_0.mean(1);
        return *mean_01.get_value(0);        
    }

    pub fn get_value(&self, i: usize) -> &f64 {
        if i >= self.values.len() {
            panic!("Index out of bounds. Want to access vector value at [{},] but vector has size [{},].", i, self.values.len())
        } else {
            &self.values[i]
        }
    }

    pub fn get_value_mut(&mut self, i: usize) -> &mut f64 {
        if i >= self.values.len() {
            panic!("Index out of bounds. Want to access vector value at [{},] but vector has size [{},].", i, self.values.len())
        } else {
            &mut self.values[i]
        }
    }

    pub fn get_mat_value(&self, i: usize, j: usize) -> &f64 {
        if (i >= self.shape[0]) | (j >= self.shape[1]) {
            panic!("Index out of bounds. Want to access matrix value at [{},{}] but matrix has size [{},{}].", i, j, self.shape[0], self.shape[1])
        } else {
            &self.values[i * self.shape[1] + j]
        }
    }

    pub fn get_mat_value_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        if (i >= self.shape[0]) | (j >= self.shape[1]) {
            panic!("Index out of bounds. Want to access matrix value at [{},{}] but matrix has size [{},{}].", i, j, self.shape[0], self.shape[1])
        } else {
            &mut self.values[i * self.shape[1] + j]
        }
    }

    pub fn get_mat_row_values(&self, i: usize) -> Vector2D {
        if i >= self.shape[0] {
            panic!("Row index out of bounds. Want to access row {} but only {} are present.", i, self.shape[0])
        } else {
            Vector2D::new(
                self.values[(i * self.shape[1])..((i+1) * self.shape[1])].to_vec(),
                [1, self.shape[1]]
            )
        }
    }
}