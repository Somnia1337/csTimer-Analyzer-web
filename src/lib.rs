#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::uninlined_format_args)]

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod analyze;

pub mod main_wasm;

pub mod options;

pub mod parser;

pub mod record;

pub mod session;

pub mod stats;

pub mod time;
