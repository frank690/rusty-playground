// This file holds all implementations for Vector2D

use super::models::Vector2D;

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

impl std::ops::Mul<f32> for &Vector2D {
    type Output = Vector2D;

    fn mul(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &self.values {
            new_values.push(value * _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Mul<&Vector2D> for f32 {
    type Output = Vector2D;

    fn mul(self, rhs: &Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &rhs.values {
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

impl std::ops::Add<f32> for &Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &self.values {
            new_values.push(value + _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Add<&Vector2D> for f32 {
    type Output = Vector2D;

    fn add(self, rhs: &Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &rhs.values {
            new_values.push(value + self);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        if self.shape != rhs.shape {
            panic!("Can not add an Vector2D to another Vector2D of different shape.")
        }
        for (v1, v2) in self.values.iter().zip(rhs.values.iter()) {
            new_values.push(v1 + v2);
        }
        Vector2D::new(new_values, rhs.shape)
    }
}

impl std::ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        if self.shape != rhs.shape {
            panic!("Can not subtract an Vector2D to another Vector2D of different shape.")
        }
        for (v1, v2) in self.values.iter().zip(rhs.values.iter()) {
            new_values.push(v1 - v2);
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

impl std::ops::Sub<&f32> for &Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: &f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &self.values {
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

impl std::ops::Sub<f32> for &Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: f32) -> Vector2D {
        let mut new_values: Vec<f32> = vec![];
        for value in &self.values {
            new_values.push(value - _rhs);
        }
        Vector2D::new(new_values, self.shape)
    }
}

impl std::ops::Sub<Vector2D> for &f32 {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
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

impl std::ops::Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        let mut new_values = vec![];
        for v in self.values {
            new_values.push(-v);
        }
        Vector2D::new(new_values, self.shape)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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