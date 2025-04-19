extern crate console_error_panic_hook;

use instant::{Duration, Instant};
use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::analyze::{analyze_single_session, append_analysis_info, append_timings};
use crate::options::AnalysisOption;
use crate::parser::{parse_options, parse_sessions};
use crate::session::Session;

use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<Option<GlobalAnalysisState>> = const { RefCell::new(None) };
}

/// Keeps the state of an analysis between
/// every JS call to wasm functions.
struct GlobalAnalysisState {
    options: Vec<AnalysisOption>,
    sessions: Vec<Session>,
    canvas: HtmlCanvasElement,
    parsing_time: Duration,
    analysis_timer: Instant,
    session_times: Vec<(usize, Duration)>,
}

/// Converts the markdown content to HTML, a
/// more time-efficient equivalent to marked.js.
#[wasm_bindgen]
pub fn render_markdown(input: &str) -> JsValue {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    JsValue::from_str(&html_output)
}

/// Initializes the analysis, parses options and sessions.
#[wasm_bindgen]
pub fn init_analysis(
    options_txt: &[u8],
    data_txt: &[u8],
    canvas: HtmlCanvasElement,
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let options_str = String::from_utf8(options_txt.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Failed to parse options: {}", e)))?;
    let data_str = String::from_utf8(data_txt.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Failed to parse data: {}", e)))?;

    let parsing_timer = Instant::now();

    let options = parse_options(&options_str);

    let sessions = parse_sessions(&data_str);

    let parsing_time = parsing_timer.elapsed();
    let analysis_timer = Instant::now();

    let state = GlobalAnalysisState {
        options,
        sessions,
        canvas,
        parsing_time,
        analysis_timer,
        session_times: Vec::new(),
    };

    STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    Ok(())
}

/// Provides information about the dataset and parsed options.
#[wasm_bindgen]
pub fn analysis_info() -> Result<JsValue, JsValue> {
    let mut result = Err(JsValue::from_str("State not found"));

    STATE.with(|cell| {
        let mut borrow = cell.borrow_mut();
        if let Some(ref mut state) = *borrow {
            let mut chunk = Vec::new();

            match append_analysis_info(&mut chunk, &state.sessions, &state.options) {
                Ok(empty) => match String::from_utf8(chunk) {
                    Ok(mut markdown) => {
                        if empty {
                            markdown.push_str("\n**Analysis aborted.**");
                        }
                        result = Ok(JsValue::from_str(&markdown));
                    }
                    Err(e) => {
                        result = Err(JsValue::from_str(&format!("UTF-8 error: {}", e)));
                    }
                },
                Err(e) => {
                    result = Err(JsValue::from_str(&format!(
                        "Failed to parse sessions or options: {}",
                        e
                    )));
                }
            }
        }
    });

    result
}

/// Provides number of sessions to be analyzed.
#[wasm_bindgen]
pub fn get_session_count() -> usize {
    STATE.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|s| s.sessions.len())
            .unwrap_or(0)
    })
}

/// Provides the analysis for the session specified by JS.
#[wasm_bindgen]
pub fn analyze_session(index: usize) -> Result<JsValue, JsValue> {
    let mut result = Err(JsValue::from_str("State not found"));

    STATE.with(|cell| {
        let mut borrow = cell.borrow_mut();
        if let Some(ref mut state) = *borrow {
            if index >= state.sessions.len() {
                result = Err(JsValue::from_str("Invalid session index"));
                return;
            }

            let session = &state.sessions[index];
            let mut chunk = Vec::new();

            match analyze_single_session(session, &state.options, &mut chunk, &state.canvas) {
                Ok(duration) => {
                    state.session_times.push((session.rank(), duration));
                    let markdown = String::from_utf8(chunk)
                        .map_err(|e| JsValue::from_str(&format!("UTF-8 error: {}", e)));
                    result = markdown.map(JsValue::from);
                }
                Err(e) => {
                    result = Err(JsValue::from_str(&format!(
                        "Failed to analyze session: {}",
                        e
                    )));
                }
            }
        }
    });

    result
}

/// Provides debug information about analysis timings.
#[wasm_bindgen]
pub fn get_timings() -> Result<JsValue, JsValue> {
    let mut result = Err(JsValue::from_str("Analysis state not initialized"));

    STATE.with(|cell| {
        let mut maybe_state = cell.borrow_mut();

        if let Some(state) = maybe_state.take() {
            let mut chunk = Vec::new();

            if let Err(e) = append_timings(
                &mut chunk,
                state.parsing_time,
                &state.session_times,
                state.analysis_timer.elapsed(),
            ) {
                result = Err(JsValue::from_str(&format!(
                    "Failed to append timings: {}",
                    e
                )));
                return;
            }

            match String::from_utf8(chunk) {
                Ok(markdown) => result = Ok(JsValue::from_str(&markdown)),
                Err(e) => result = Err(JsValue::from_str(&format!("UTF-8 error: {}", e))),
            }
        }
    });

    result
}
