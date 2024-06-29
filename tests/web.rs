//! Test suite for the Web and headless browsers.
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate wasm_pack_simd_template;
use wasm_pack_simd_template::dot_4x4;
#[wasm_bindgen_test]
fn test_dot_4x4() {
    #[rustfmt::skip]
    let p1 = [
        1., 2., 3., 4.,
        5., 6., 7., 8.,
        1., 2., 3., 4.,
        5., 6., 7., 8.
    ];

    let ret = dot_4x4(&p1, &p1);

    #[rustfmt::skip]
    let expected = vec![
        34., 44., 54., 64., 
        82., 108., 134., 160., 
        34., 44., 54., 64., 
        82., 108., 134., 160.,
    ];

    assert_eq!(ret, expected);
}
