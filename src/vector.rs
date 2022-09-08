// This file contains math functions regarding vectors that are commonly used across this project

use std::f64::INFINITY;

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