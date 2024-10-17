#![allow(dead_code)]

use phantom::dev::run_e2e;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn e2e() {
    run_e2e();
}
