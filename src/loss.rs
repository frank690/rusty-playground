// This file contains loss functions that can be used by the neural network
use crate::vectors::models::Vector2D;

pub fn cross_entropy_loss(mut h: Vector2D, mut y: Vector2D) -> Vector2D {
    (-1. / y.values.len() as f32) * (
        (y.transpose().dot(&h.ln())) +
        ((1. - &y.transpose()).dot(&(1. - &h).ln()))
    )
}

pub fn cross_entropy_derivative(h: Vector2D, y: Vector2D) -> Vector2D {
    -(&y / &h) + (1. - y) / (1. - h)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_entropy_loss() {
        let h: Vector2D = Vector2D::new(
            vec![0., 0., 0.1, 0.1, 0.25, 0.25, 0.5, 0.5, 0.65, 0.65, 0.8, 0.8, 1., 1.],
            [14, 1]
        );
        let y: Vector2D = Vector2D::new(
            vec![0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.],
            [14, 1]
        );
        let r: Vector2D = cross_entropy_loss(h, y);
        assert!(r.shape == [1, 1]);
        assert!(r.values[0].is_infinite());

        let h: Vector2D = Vector2D::new(
            vec![0., 0.1, 0.1, 0.1, 0.25, 0.25, 0.5, 0.5, 0.65, 0.65, 0.8, 0.8, 1., 1.],
            [14, 1]
        );        
        let y: Vector2D = Vector2D::new(
            vec![0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.],
            [14, 1]
        );
        let r: Vector2D = cross_entropy_loss(h, y);
        assert!(r.shape == [1, 1]);
        assert!(r.values[0].is_infinite());

        let h: Vector2D = Vector2D::new(
            vec![0., 0.1, 0.1, 0.1, 0.25, 0.25, 0.5, 0.5, 0.65, 0.65, 0.8, 0.8, 0.9, 1.],
            [14, 1]
        ); 
        let y: Vector2D = Vector2D::new(
            vec![0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.],
            [14, 1]
        );
        let r: Vector2D = cross_entropy_loss(h, y);
        assert!(r.shape == [1, 1]);
        assert!(r.values[0] <= 0.9562 as f32);
        assert!(r.values[0] >= 0.9561 as f32);
    }

    #[test]
    fn test_cross_entropy_loss_derivative() {
        let h: Vector2D = Vector2D::new(vec![0., 1.], [2, 1]);
        let y: Vector2D = Vector2D::new(vec![0., 1.], [2, 1]);
        let d: Vector2D = cross_entropy_derivative(h, y);
        for e in d.values {
            assert!(e.is_nan());
        }

        let h: Vector2D = Vector2D::new(vec![0.1, 0.1], [2, 1]);
        let y: Vector2D = Vector2D::new(vec![1., 0.], [2, 1]);
        let d: Vector2D = cross_entropy_derivative(h, y);
        assert!(d[0] >= -10.001);
        assert!(d[0] <= -9.999);
        assert!(d[1] >= 1.111);
        assert!(d[1] <= 1.112);

        let h: Vector2D = Vector2D::new(vec![0.8, 0.8], [1, 2]);
        let y: Vector2D = Vector2D::new(vec![1., 0.], [1, 2]);
        let d: Vector2D = cross_entropy_derivative(h, y);
        assert!(d[0] >= -1.251);
        assert!(d[0] <= -1.249);
        assert!(d[1] <= 5.001);
        assert!(d[1] >= 4.999);
    }
}
