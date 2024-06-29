#![feature(portable_simd)]
use std::simd::*;

mod utils;

use wasm_bindgen::prelude::*;

pub fn dot_4x4_simd(left: &[f32; 16], right: &[f32; 16]) -> [f32; 16] {
    // Create result vector initialized with zeros
    let mut result = [0.0; 16];

    // Load right matrix rows into SIMD vectors
    let b0 = f32x4::from_slice(&right[0..4]);
    let b1 = f32x4::from_slice(&right[4..8]);
    let b2 = f32x4::from_slice(&right[8..12]);
    let b3 = f32x4::from_slice(&right[12..16]);

    // Perform matrix multiplication
    for i in 0..4 {
        let row_offset = i * 4;
        let a0 = f32x4::splat(left[row_offset + 0]);
        let a1 = f32x4::splat(left[row_offset + 1]);
        let a2 = f32x4::splat(left[row_offset + 2]);
        let a3 = f32x4::splat(left[row_offset + 3]);

        // Compute dot product for each row of left matrix with columns of right matrix
        let r0 = a0 * b0 + a1 * b1 + a2 * b2 + a3 * b3;
        // Store results back to the result array
        r0.copy_to_slice(&mut result[row_offset..row_offset + 4]);
    }

    result
}

pub fn dot_4x4_naive(left: &[f32; 16], right: &[f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            result[i * 4 + j] = 0.0;
            for k in 0..4 {
                result[i * 4 + j] += left[i * 4 + k] * right[k * 4 + j];
            }
        }
    }
    result
}

#[wasm_bindgen]
pub fn dot4x4_simd(left: &[f32], right: &[f32]) -> Vec<f32> {
    // Ensure the input slices have the correct length
    assert_eq!(left.len(), 16);
    assert_eq!(right.len(), 16);

    return dot_4x4_simd(left.try_into().unwrap(), right.try_into().unwrap()).to_vec();
}

#[wasm_bindgen]
pub fn dot4x4_naive(left: &[f32], right: &[f32]) -> Vec<f32> {
    // Ensure the input slices have the correct length
    assert_eq!(left.len(), 16);
    assert_eq!(right.len(), 16);

    return dot_4x4_naive(left.try_into().unwrap(), right.try_into().unwrap()).to_vec();
}
