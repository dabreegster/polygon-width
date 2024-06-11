use std::sync::Once;

use anyhow::{bail, Result};

use wasm_bindgen::prelude::*;

static START: Once = Once::new();

/// Takes GeoJSON with polygons in WGS84
#[wasm_bindgen(js_name = findWidths)]
pub fn find_widths(input: String) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    Ok(format!("got text input {}", input.len()))
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
