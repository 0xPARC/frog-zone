use phantom::dev::run_e2e;
use wasm_bindgen_test::*;

#[wasm_bindgen_test(unsupported = test)]
fn e2e() {
    run_e2e();
}
