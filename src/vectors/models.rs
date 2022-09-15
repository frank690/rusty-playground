// This file contains all implementation of custom vector models (structs)

#[derive(Clone)]
pub struct Vector2D {
    pub values: Vec<f32>,
    pub shape: [usize; 2]
}

impl Vector2D {
    pub fn new(values: Vec<f32>, shape: [usize; 2]) -> Vector2D {
        if values.len() != shape[0] * shape[1] {
            panic!("Shape {:?} doesn't match length {}", shape, values.len());
        }
        Vector2D { values, shape }
    }

    pub fn transpose(&mut self) -> Vector2D {
        let new_shape: [usize; 2] = [self.shape[1], self.shape[0]];
        Vector2D::new(self.values.to_vec(), new_shape)    
    }

    pub fn dot(&mut self, b_vector: &Vector2D) -> Vector2D {
        if self.shape[1] != b_vector.shape[0] {
            panic!("Can not dot multiply vectors with shape {:?} @ {:?}", self.shape, b_vector.shape);
        } else {
            let mut result: Vec<f32> = vec![];
            let result_shape: [usize; 2] = [self.shape[0], b_vector.shape[1]];

            for row in 0..result_shape[0] {
                for col in 0..result_shape[1] {
                    let mut value: f32 = 0.;
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

    pub fn ln(&mut self) -> Vector2D {
        let mut log_values = vec![];
        for v in &self.values {
            log_values.push(f32::ln(*v));
        }
        Vector2D::new(log_values, self.shape)
    }

    pub fn get_value(&self, i: usize) -> &f32 {
        if i >= self.values.len() {
            panic!("Index out of bounds.")
        } else {
            &self.values[i]
        }
    }

    pub fn get_value_mut(&mut self, i: usize) -> &mut f32 {
        if i >= self.values.len() {
            panic!("Index out of bounds.")
        } else {
            &mut self.values[i]
        }
    }

    pub fn get_mat_value(&self, i: usize, j: usize) -> &f32 {
        if (i >= self.shape[0]) | (j >= self.shape[1]) {
            panic!("Index out of bounds.")
        } else {
            &self.values[i * self.shape[1] + j]
        }
    }

    pub fn get_mat_value_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        if (i >= self.shape[0]) | (j >= self.shape[1]) {
            panic!("Index out of bounds.")
        } else {
            &mut self.values[i * self.shape[1] + j]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
            let values = vec![0., 1., 2., 3.];
            let values_copy = &values.to_vec();
            let shape = [2, 2];
            let v2d: Vector2D = Vector2D::new(values, shape);
            assert!(v2d.shape == shape);
            assert!(&v2d.values == values_copy);
    }

    #[test]
    fn test_transpose() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let expected_values = &values.to_vec();
        let shape = [2, 3];
        let expected_shape = [3, 2];
        let mut v2d: Vector2D = Vector2D::new(values, shape);
        assert!(&v2d.transpose().values == expected_values);
        assert!(v2d.transpose().shape == expected_shape);
    }

    #[test]
    fn test_dot() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);

        let values = vec![0.5, 1., 2.];
        let shape = [3, 1];
        let v2: Vector2D = Vector2D::new(values, shape);

        let v3 = v1.dot(&v2);
        assert!(v3.values == vec![5., 15.5]);
        assert!(v3.shape == [2, 1]);
    }

    #[test]
    fn test_ln() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);
        let v2 = v1.ln();
        assert!(v2.shape == v1.shape);
        assert!(v2.values[0].is_infinite());
        assert!(v2.values[1] == 0.);
    }

    #[test]
    fn test_get_value() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let v1: Vector2D = Vector2D::new(values, shape);
        assert!(v1.values[0] == *v1.get_value(0));
        assert!(v1.values[4] == *v1.get_value(4));
    }

    #[test]
    fn test_get_value_mut() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);
        assert!(v1.values[0] == *v1.get_value_mut(0));
        assert!(v1.values[4] == *v1.get_value_mut(4));
    }

    #[test]
    fn test_get_mat_value() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let v1: Vector2D = Vector2D::new(values, shape);
        assert!(v1.values[0] == *v1.get_mat_value(0, 0));
        assert!(v1.values[3] == *v1.get_mat_value(1, 0));
        assert!(v1.values[5] == *v1.get_mat_value(1, 2));
    }

    #[test]
    fn test_get_mat_value_mut() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);
        assert!(v1.values[0] == *v1.get_mat_value_mut(0, 0));
        assert!(v1.values[3] == *v1.get_mat_value_mut(1, 0));
        assert!(v1.values[5] == *v1.get_mat_value_mut(1, 2));
    }
}