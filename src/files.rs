use regex::Regex;

use crate::record::Record;
use crate::session::Session;
use crate::types::*;

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
