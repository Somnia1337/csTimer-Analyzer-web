use crate::options::AnalysisOption;
use crate::record::{Record, SolveState};
use crate::session::Session;
use crate::time::Milliseconds;

use serde_json::Value;
use web_sys::js_sys::Date;

// Gets the local offset from UTC in seconds.
fn local_offset_seconds() -> i64 {
    -Date::new_0().get_timezone_offset() as i64 * 60
}

/// Parses `Session`s and `Record`s within.
pub fn parse_sessions(input: &str) -> Vec<Session> {
    let data: Value = match serde_json::from_str(input) {
        Ok(json) => json,
        Err(_) => {
            return Vec::new();
        }
    };

    let session_metadata = parse_session_metadata(&data);

    let mut sessions = Vec::new();
    if let Some(obj) = data.as_object() {
        let offset = local_offset_seconds();

        for (key, value) in obj {
            if key.starts_with("session") {
                if let Some(id) = key
                    .strip_prefix("session")
                    .and_then(|id| id.parse::<usize>().ok())
                {
                    let records = parse_records(value, offset);
                    if records.is_empty() {
                        continue;
                    }

                    if let Some((_, name, rank, date_time)) =
                        session_metadata.iter().find(|(sid, _, _, _)| *sid == id)
                    {
                        sessions.push(Session::from(
                            *rank,
                            name.clone(),
                            (date_time.0 + offset, date_time.1 + offset),
                            &records,
                        ));
                    }
                }
            }
        }
    }

    sessions.sort_unstable_by_key(super::session::Session::rank);

    sessions
}

/// Parses `Record`s in a `Session`.
pub fn parse_records(session: &Value, offset: i64) -> Vec<Record> {
    session
        .as_array()
        .iter()
        .next()
        .unwrap_or(&&Vec::new())
        .iter()
        .filter_map(|r| {
            let mut solve_state = match r.get(0)?.get(0)?.as_i64()? {
                0 => SolveState::Ok,
                2000 => SolveState::Plus2,
                -1 => SolveState::Dnf,
                _ => return None,
            };

            let mut time_millis = r.get(0)?.get(1)?.as_i64()?;
            if time_millis < 0 {
                time_millis = -time_millis;
                solve_state = SolveState::Dnf;
            } else if solve_state.is_plus2() {
                time_millis += 2000;
            }

            let scramble = r.get(1)?.as_str()?.to_string();
            let comment = r
                .get(2)?
                .as_str()?
                .to_string()
                .trim()
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
                .replace('*', "\\*");
            let time_epoch = r.get(3)?.as_i64()?;

            Some(Record::from(
                solve_state,
                time_millis as Milliseconds,
                scramble,
                comment,
                time_epoch + offset,
            ))
        })
        .collect()
}

/// Parses metadata for every `Session`.
pub fn parse_session_metadata(json: &Value) -> Vec<(usize, String, usize, (i64, i64))> {
    let session_data = json.get("properties").and_then(|p| p.get("sessionData"));
    if session_data.is_none() {
        return Vec::new();
    }

    let data_str = session_data
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or("{}");
    let data: Value = match serde_json::from_str(data_str) {
        Ok(json) => json,
        Err(_) => {
            return Vec::new();
        }
    };

    let mut session_data = Vec::new();

    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let id: usize = key.parse().unwrap_or_default();
            let name = value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let rank = value
                .get("rank")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or_default() as usize;

            let date1 = value
                .get("date")
                .and_then(|v| v.get(0))
                .and_then(serde_json::Value::as_i64)
                .unwrap_or_default();
            let date2 = value
                .get("date")
                .and_then(|v| v.get(1))
                .and_then(serde_json::Value::as_i64)
                .unwrap_or_default();

            session_data.push((id, name, rank, (date1, date2)));
        }
    }

    session_data
}

/// Applies sanitization to the options string.
fn sanitize_options(options: &str) -> Vec<String> {
    options
        .lines()
        .map(|op| {
            let end = op.to_string().find('#').unwrap_or(op.len());
            op[0..end].trim().to_lowercase()
        })
        .filter(|op| !op.is_empty())
        .collect()
}

/// Parses options and removes duplicates.
pub fn parse_options(options: &str) -> Vec<AnalysisOption> {
    let options = sanitize_options(options);

    let mut seen = std::collections::HashSet::with_capacity(options.len());
    let options: Vec<AnalysisOption> = options
        .into_iter()
        .filter_map(|s| AnalysisOption::try_from(s.as_str()).ok())
        .filter(|s| seen.insert(s.clone()))
        .collect();

    options
}
