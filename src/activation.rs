// This file contains activation functions

pub fn sigmoid(v: &f32) -> f32 {
    1. / (1. + (-v).exp())
}

pub fn sigmoid_derivative(v: &f32) -> f32 {
    let sig = sigmoid(v);
    sig * (1. - sig)
}

pub fn vector_sigmoid(v: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(sigmoid(e));
    }
    return r;
}

pub fn vector_sigmoid_derivative(v: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(sigmoid_derivative(e));
    }
    return r;
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_sigmoid() {
        assert!(sigmoid(&0.) == 0.5);
        assert!(sigmoid(&-0.) == 0.5);
        assert!(sigmoid(&0.00001) > 0.5);
        assert!(sigmoid(&-0.00001) < 0.5);     
    }

    #[test]
    fn test_sigmoid_derivative() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let rnd: f32 = 5. * rng.gen::<f32>();
            assert!(sigmoid_derivative(&rnd) > 0.);
        }

        assert!(sigmoid_derivative(&1000000.) == 0.);
        assert!(sigmoid_derivative(&-1000000.) == 0.);
    }
}