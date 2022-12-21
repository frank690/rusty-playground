
#[cfg(test)]
mod tests {
    use crate::vectors::models::Vector2D;


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
    fn test_transpose_mat() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let expected_values = &vec![0., 3., 1., 4., 2., 5.];
        let shape = [2, 3];
        let expected_shape = [3, 2];
        let v2d: Vector2D = Vector2D::new(values, shape);
        assert!(&v2d.transpose().values == expected_values);
        assert!(v2d.transpose().shape == expected_shape);
    }

    #[test]
    fn test_transpose_vec() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let expected_values = &values.clone();
        let shape = [6, 1];
        let expected_shape = [1, 6];
        let v2d: Vector2D = Vector2D::new(values, shape);
        assert!(&v2d.transpose().values == expected_values);
        assert!(v2d.transpose().shape == expected_shape);
    }

    #[test]
    fn test_transpose_scalar() {
        let values = vec![42.];
        let expected_values = &values.clone();
        let shape = [1, 1];
        let expected_shape = [1, 1];
        let v2d: Vector2D = Vector2D::new(values, shape);
        assert!(&v2d.transpose().values == expected_values);
        assert!(v2d.transpose().shape == expected_shape);
    }

    #[test]
    fn test_row_add() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);

        let values = vec![0.5, 1., 2.];
        let shape = [1, 3];
        let v2: Vector2D = Vector2D::new(values, shape);

        let v3 = v1.row_add(&v2);
        assert!(v3.values == vec![0.5, 2., 4., 3.5, 5., 7.]);
        assert!(v3.shape == [2, 3]);
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
        let v1: Vector2D = Vector2D::new(values, shape);
        let v2 = v1.ln();
        assert!(v2.shape == v1.shape);
        assert!(v2.values[0].is_infinite());
        assert!(v2.values[1] == 0.);
    }

    #[test]
    fn test_mean() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);

        let mean_0 = v1.mean(0).values;
        assert!(mean_0.len() == shape[1]);
        assert!(mean_0 == vec![1.5, 2.5, 3.5]);

        let mean_1 = v1.mean(1).values;
        assert!(mean_1.len() == shape[0]);
        assert!(mean_1 == vec![1., 4.]);
    }

    #[test]
    fn test_overall_mean() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let mut v1: Vector2D = Vector2D::new(values, shape);

        let overall_mean = v1.overall_mean();
        assert!(overall_mean == 2.5);
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

    #[test]
    fn test_get_mat_row_values() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let v1: Vector2D = Vector2D::new(values, shape);

        let v10: Vector2D = v1.get_mat_row_values(0);
        assert!(vec![0., 1., 2.] == v10.values);
        assert!([1, 3] == v10.shape);

        let v11: Vector2D = v1.get_mat_row_values(1);
        assert!(vec![3., 4., 5.] == v11.values);
        assert!([1, 3] == v11.shape);
    }

    #[test]
    #[should_panic(expected = "Row index out of bounds.")]
    fn test_get_mat_row_values_panicing() {
        let values = vec![0., 1., 2., 3., 4., 5.];
        let shape = [2, 3];
        let v1: Vector2D = Vector2D::new(values, shape);
        v1.get_mat_row_values(2);
    }

}