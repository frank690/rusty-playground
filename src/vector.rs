// This file contains math functions regarding vectors that are commonly used across this project

pub struct Vector2D {
    values: Vec<f32>,
    shape: [u32; 2]
}

impl Vector2D {
    pub fn new(values: Vec<f32>, shape: [u32; 2]) -> Vector2D {
        Vector2D { values: values, shape: shape }
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

pub fn scalar_multiply(v: &Vec<f32>, s: &f32) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(e * s);
    }
    return r;
}

pub fn scalar_add(v: &Vec<f32>, s: &f32) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(e + s);
    }
    return r;
}

pub fn vector_multiply(v: &Vec<f32>, w: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];

    if v.len() == w.len() {
        for (vi, wi) in v.iter().zip(w.iter()) {
            r.push(vi * wi);
        }
    }

    return r;
}

pub fn vector_divide(v: &Vec<f32>, w: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];

    if v.len() == w.len() {
        for (vi, wi) in v.iter().zip(w.iter()) {
            r.push(vi / wi);
        }
    }

    return r;
}

pub fn dot(v: &Vec<f32>, w: &Vec<f32>) -> f32 {
    let mut r: f32 = 0.;

    if v.len() == w.len() {
        for (vi, wi) in v.iter().zip(w.iter()) {
            if (vi == &0.) | (wi == &0.) { continue; }
            r += vi * wi;
        }
    }

    return r;
}

pub fn vector_add(v: &Vec<f32>, w: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];

    if v.len() == w.len() {
        for (vi, wi) in v.iter().zip(w.iter()) {
            r.push(vi + wi);
        }
    }

    return r;
}

pub fn logarithm(v: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(f32::ln(*e));
    }
    return r;
}

pub fn negate(v: &Vec<f32>) -> Vec<f32> {
    let mut r: Vec<f32> = vec![];
    for e in v {
        r.push(-e);
    }
    return r;
}