// This file contains all implementation of custom vector models (structs)

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
        let new_shape: [usize; 2] = [self.shape[0], self.shape[1]];
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
                        value += self.get_mat_value(row, idx) * b_vector.get_mat_value(idx, col);
                    }
                    result.push(value);
                }
            }
            Vector2D::new(result, result_shape)
        }
    }

    pub fn get_value(&self, i: usize) -> &f32 {
        if i >= self.values.len() {
            panic!("Index out of bounds.")
        } else {
            &self.values[i]
        }
    }

    pub fn get_mat_value(&self, i: usize, j: usize) -> &f32 {
        if (i >= self.shape[0]) | (j >= self.shape[1]) {
            panic!("Index out of bounds.")
        } else {
            &self.values[i * self.shape[1] + j]
        }
    }

    pub fn get_value_mut(&mut self, i: usize) -> &mut f32 {
        if i >= self.values.len() {
            panic!("Index out of bounds.")
        } else {
            &mut self.values[i]
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

impl std::ops::Index<usize> for Vector2D {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        self.get_value(i)
    }
}

impl std::ops::Index<(usize, usize)> for Vector2D {
    type Output = f32;
    fn index(&self, (i, j): (usize, usize)) -> &f32 {
        self.get_mat_value(i, j)
    }
}

impl std::ops::IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        self.get_value_mut(i)
    }
}

impl std::ops::IndexMut<(usize, usize)> for Vector2D {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
        self.get_mat_value_mut(i, j)
    }
}

impl std::ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in self.values {
            new_values.push(value * _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Mul<Vector2D> for f32 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in rhs.values {
            new_values.push(value * self);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Add<f32> for Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in self.values {
            new_values.push(value + _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Add<Vector2D> for f32 {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in rhs.values {
            new_values.push(value + self);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Sub<f32> for Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in self.values {
            new_values.push(value - _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Sub<Vector2D> for f32 {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in rhs.values {
            new_values.push(value - self);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Sub<&f32> for Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: &f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in self.values {
            new_values.push(value - _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Sub<&Vector2D> for f32 {
    type Output = Vector2D;

    fn sub(self, rhs: &Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &rhs.values {
            new_values.push(value - self);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(mut self, rhs: Vector2D) -> Vector2D {
        self.dot(&rhs)
    }
}

impl std::ops::Mul<&Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(mut self, rhs: &Vector2D) -> Vector2D {
        self.dot(rhs)
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
    fn test_indexing() {
        let values: Vec<f32> = vec![0., 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7];
        let shape: [usize; 2] = [4, 2];
    
        let v2d: Vector2D = Vector2D { values, shape };
        assert!(v2d[5] == 5.5);
        assert!(v2d[(3, 1)] == 7.7);
        assert!(v2d[(0, 0)] == 0.);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds.")]
    fn test_indexing_panicing() {
        let values: Vec<f32> = vec![0., 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7];
        let shape: [usize; 2] = [4, 2];
    
        let v2d: Vector2D = Vector2D { values, shape };
        println!("{}", v2d[42]);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds.")]
    fn test_matrix_indexing_panicing() {
        let values: Vec<f32> = vec![0., 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7];
        let shape: [usize; 2] = [4, 2];
    
        let v2d: Vector2D = Vector2D { values, shape };
        println!("{}", v2d[(13, 1)]);
    }

    #[test]
    fn test_multiplications() {
        let v: f32 = 1.5;
        let values: Vec<f32> = vec![0., 1., 2., 3.];
        let values_copy = &values.to_vec();

        let expected_result: Vec<f32> = vec![0., 1.5, 3., 4.5];
        let shape: [usize; 2] = [2, 2];
        let v2d: Vector2D = Vector2D::new(values, shape);
        let result: Vector2D = v2d * v;
        assert!(&result.values == &expected_result);
        let v2d: Vector2D = (1./v) * result;
        assert!(&v2d.values == values_copy);
    }
}
