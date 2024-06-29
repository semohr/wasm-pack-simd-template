#![feature(portable_simd)]
use std::simd::*;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn dot_4x4(left: &[f32], right: &[f32]) -> Vec<f32> {
    // Ensure the input slices have the correct length
    assert_eq!(left.len(), 16);
    assert_eq!(right.len(), 16);

    // Create result vector initialized with zeros
    let mut result = Vec::with_capacity(16);
    // Load right matrix rows into SIMD vectors
    let bcx = f32x4::from_slice(&right[0..4]);
    let bcy = f32x4::from_slice(&right[4..8]);
    let bcz = f32x4::from_slice(&right[8..12]);
    let bcw = f32x4::from_slice(&right[12..16]);

    // Perform matrix multiplication
    for i in 0..4 {
        let row_offset = i * 4;
        let arx = f32x4::splat(left[row_offset]);
        let ary = f32x4::splat(left[row_offset + 1]);
        let arz = f32x4::splat(left[row_offset + 2]);
        let arw = f32x4::splat(left[row_offset + 3]);

        let x = arx * bcx;
        let y = ary * bcy;
        let z = arz * bcz;
        let w = arw * bcw;

        let r = x + y + z + w;

        result.extend_from_slice(&r.to_array());
    }

    result
}
