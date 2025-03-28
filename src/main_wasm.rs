extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::analyze::*;
use crate::json::*;
use crate::options::*;

#[wasm_bindgen]
pub fn analyze_from_files(
    options_txt: &[u8],
    data_txt: &[u8],
    canvas: HtmlCanvasElement,
) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    // Convert to String
    let options = String::from_utf8(options_txt.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Failed to parse options: {}", e)))?;
    let data = String::from_utf8(data_txt.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Failed to parse data: {}", e)))?;

    // Reads and parses options
    let options = parse_options(options);

    // Splits sessions
    let sessions = split_sessions(&data);

    // Analyzes sessions
    let mut output = Vec::new();
    analyze(&sessions, &options, &mut output, canvas)
        .map_err(|e| JsValue::from_str(&format!("Failed to analyze sessions: {}", e)))?;

    // Return markdown result
    let analysis = String::from_utf8(output)
        .map_err(|e| JsValue::from_str(&format!("Failed to convert analysis to UTF-8: {}", e)))?;

    Ok(JsValue::from_str(&analysis))
}
