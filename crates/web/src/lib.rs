use wasm_bindgen::prelude::*;

// TODO: This create is a JsValue interface with serde_wasm_bindgen

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct MakuWeb {}

#[wasm_bindgen]
impl MakuWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MakuWeb {
        MakuWeb {}
    }
}
