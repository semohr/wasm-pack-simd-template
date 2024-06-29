//! Test suite for the Web and headless browsers.
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate wasm_pack_simd_template;
use wasm_pack_simd_template::{dot4x4_naive, dot4x4_simd, dot_4x4_naive, dot_4x4_simd};
#[wasm_bindgen_test]
fn test_dot_4x4() {
    #[rustfmt::skip]
    let p1 = [
        1., 2., 3., 4.,
        5., 6., 7., 8.,
        1., 2., 3., 4.,
        5., 6., 7., 8.
    ];

    #[rustfmt::skip]
    let expected =[
        34., 44., 54., 64., 
        82., 108., 134., 160., 
        34., 44., 54., 64., 
        82., 108., 134., 160.,
        ];

    let ret = dot4x4_simd(&p1, &p1);
    assert_eq!(ret, expected);

    let ret = dot4x4_naive(&p1, &p1);
    assert_eq!(ret, expected);
}

mod bench;

#[wasm_bindgen_test]
fn bench_dot_4x4() {
    console_log!("4x4 * 4x4 (naive, bindgen)");
    bench::measure_execution_time(compute_dot_bind, 1000, 1000);
    console_log!("4x4 * 4x4 (simd, bindgen)");
    bench::measure_execution_time(compute_dot_simd_bind, 1000, 1000);
    console_log!("4x4 * 4x4 (naive)");
    bench::measure_execution_time(compute_dot, 1000, 1000);
    console_log!("4x4 * 4x4 (simd)");
    bench::measure_execution_time(compute_dot_simd, 1000, 1000);
}

const M1: [f32; 16] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
];

const M2: [f32; 16] = [
    17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0,
];

// Function to compute 1000 4x4 dot products using [f32; 16] arrays
fn compute_dot_bind() {
    dot4x4_naive(&M1, &M2);
}

fn compute_dot() {
    dot_4x4_naive(&M1, &M2);
}

fn compute_dot_simd_bind() {
    dot4x4_simd(&M1, &M2);
}

fn compute_dot_simd() {
    dot_4x4_simd(&M1, &M2);
}
