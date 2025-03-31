extern crate console_error_panic_hook;

use instant::Instant;
use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::analyze::analyze;
use crate::json::split_sessions;
use crate::options::parse_options;

// todo: unwrap()s

// todo: leverage browser cache to use without internet

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

    let parsing_timer = Instant::now();

    // Reads and parses options
    let options = parse_options(&options);

    // Splits sessions
    let sessions = split_sessions(&data);
    // todo

    let parsing_time = parsing_timer.elapsed();

    // Analyzes sessions
    let mut output = Vec::new();
    analyze(&sessions, &options, &mut output, &canvas, parsing_time)
        .map_err(|e| JsValue::from_str(&format!("Failed to analyze sessions: {}", e)))?;

    // Return markdown result
    let analysis = String::from_utf8(output)
        .map_err(|e| JsValue::from_str(&format!("Failed to convert analysis to UTF-8: {}", e)))?;

    Ok(JsValue::from_str(&analysis))
}

#[wasm_bindgen]
pub fn render_markdown(input: &str) -> JsValue {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    JsValue::from_str(&html_output)
}
