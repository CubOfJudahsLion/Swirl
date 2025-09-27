use wasm_bindgen::prelude::*;
use js_sys::Math;
use web_sys::console;

#[derive(Debug)]
#[repr(C)]
struct TwoInt32s {
    a: i32,
    b: i32,
}

extern "C" {
    fn add_c(a: i32, b: i32) -> i32;
    fn get_num_and_next() -> TwoInt32s;
}

#[no_mangle]
pub extern "C" fn gimme_a_number() -> i32 {
    (Math::random() * i32::MAX as f64) as i32
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&JsValue::from_str(&format!("{} + {} = {}", 1, 2, unsafe { add_c(1, 2) })));
    console::log_1(&JsValue::from_str(&format!("{:?}", unsafe { get_num_and_next() })));
}
