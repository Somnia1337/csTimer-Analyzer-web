use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

use regex::Regex;

use crate::errors::*;
use crate::record::Record;
use crate::session::Session;
use crate::types::*;

/// Reads a text file.
pub fn read_txt(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

/// Tries to match a single csTimer exported data file.
pub fn match_data_file() -> Result<String, DataFileMatchError> {
    let dir = Path::new(".");
    let re = Regex::new(r"^cstimer_\d{8}_\d{6}\.txt$").unwrap();
    let mut data_file_path = String::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if re.is_match(file_name) {
                    if data_file_path.is_empty() {
                        data_file_path = file_name.to_string();
                    } else {
                        return Err(DataFileMatchError::Duplicate);
                    }
                }
            }
        }
    }

    if data_file_path.is_empty() {
        Err(DataFileMatchError::NotFound)
    } else {
        Ok(data_file_path)
    }
}

/// Splits sessions and parse every record within.
pub fn split_sessions(input: &str) -> Vec<Session> {
    let re = Regex::new(r#"\"session(\d+)\":\[(\[.*?\])\]"#).unwrap();

    let mut sessions = Vec::new();
    for cap in re.captures_iter(input) {
        let id: u8 = cap[1].parse().unwrap();
        let records = extract_records(&cap[2]);

        sessions.push(Session::new(id, &records));
    }

    sessions
}

/// Parses records in a session.
pub fn extract_records(input: &str) -> Vec<Record> {
    let re =
        Regex::new(r#"\[\s*(-?\d+)\s*,\s*(\d+)\s*\],\s*\"([^\"]+?)\",\"((?:\\\"|[^\"])*)\",(\d+)"#)
            .unwrap();

    let mut records = Vec::new();
    for cap in re.captures_iter(input) {
        let solve_state = match cap[1].parse::<i32>().unwrap() {
            0 => SolveState::Ok,
            2000 => SolveState::Plus2,
            -1 => SolveState::Dnf,
            _ => unreachable!("Unknown solve state"),
        };

        let mut time_millis = cap[2].parse::<Milliseconds>().unwrap();
        if solve_state.is_plus2() {
            time_millis += 2000;
        }

        let scramble = cap[3].to_string();

        let comment = cap[4]
            .to_string()
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");

        let time_epoch = cap[5].parse::<i64>().unwrap();

        records.push(Record::new(
            solve_state,
            time_millis,
            scramble,
            comment,
            time_epoch,
        ));
    }

    records
}

/// Creates directories and report file for analysis.
pub fn create_dir_and_file(dir: &str) -> io::Result<File> {
    std::fs::create_dir(dir)?;
    std::fs::create_dir(format!("{}/images", dir))?;

    File::create(format!("{}/Analysis.md", dir))
}
