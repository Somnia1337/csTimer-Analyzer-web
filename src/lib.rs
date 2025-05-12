#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod analyze;

pub mod parser;

pub mod main_wasm;

pub mod options;

pub mod record;

pub mod session;

pub mod stats;

pub mod time;
