// This file contains math functions regarding vectors that are commonly used across this project

pub fn scalar_multiply(vector: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for element in vector {
        result.push(element * scalar);
    }
    return result;
}

pub fn logarithm(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for element in vector {
        result.push(f32::ln(*element));
    }
    return result;
}