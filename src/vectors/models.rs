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

    pub fn dot(&mut self, b_vector: Vector2D) -> Vector2D {
        if self.shape[1] != b_vector.shape[0] {
            panic!("Can not dot multiply vectors with shape {:?} @ {:?}", self.shape, b_vector.shape);
        } else {
            Vector2D::new(self.values.to_vec(), self.shape)
        }
    }

    pub fn get_value(&self, i: usize) -> Option<&f32> {
        self.values.get(i)
    }

    pub fn get_mat_value(&self, i: usize, j: usize) -> Option<&f32> {
        self.values.get(i * self.shape[1] + j)
    }

    pub fn get_value_mut(&mut self, i: usize) -> Option<&mut f32> {
        self.values.get_mut(i)
    }

    pub fn get_mat_value_mut(&mut self, i: usize, j: usize) -> Option<&mut f32> {
        self.values.get_mut(i * self.shape[1] + j)
    }
}

impl std::ops::Index<usize> for Vector2D {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        self.get_value(i).expect("Index out of bounds.")
    }
}

impl std::ops::Index<(usize, usize)> for Vector2D {
    type Output = f32;
    fn index(&self, k: (usize, usize)) -> &f32 {
        let (i, j) = k;
        self.get_mat_value(i, j).expect("Index out of bounds.")
    }
}

impl std::ops::IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        self.get_value_mut(i).expect("Index out of bounds.")
    }
}

impl std::ops::IndexMut<(usize, usize)> for Vector2D {
    fn index_mut(&mut self, k: (usize, usize)) -> &mut f32 {
        let (i, j) = k;
        self.get_mat_value_mut(i, j).expect("Index out of bounds.")
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
