use crate::record::Record;
use crate::session::Session;
use crate::types::*;
use serde_json::Value;

/// Splits sessions and parse every record within.
pub fn split_sessions(input: &str) -> Vec<Session> {
    let data: Value = match serde_json::from_str(input) {
        Ok(json) => json,
        Err(_) => {
            return vec![];
        }
    };

    let session_data = session_data(&data);

    let mut sessions = vec![];
    if let Some(obj) = data.as_object() {
        for (key, value) in obj.iter() {
            if key.starts_with("session") {
                if let Some(id) = key
                    .strip_prefix("session")
                    .and_then(|s| s.parse::<u8>().ok())
                {
                    if let Some(records) = extract_records(value) {
                        if records.is_empty() {
                            continue;
                        }
                        if let Some((_, name, rank, date_time)) =
                            session_data.iter().find(|(sid, _, _, _)| *sid == id)
                        {
                            sessions.push(Session::new(*rank, name.clone(), *date_time, &records));
                        }
                    }
                }
            }
        }
    }

    sessions.sort_unstable_by_key(|s| s.rank());

    sessions
}

/// Parses records in a session.
pub fn extract_records(session: &Value) -> Option<Vec<Record>> {
    session
        .as_array()
        .iter()
        .next()
        .unwrap()
        .iter()
        .map(|r| {
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
                .replace("*", "\\*");
            let time_epoch = r.get(3)?.as_i64()?;

            Some(Record::new(
                solve_state,
                time_millis as Milliseconds,
                scramble,
                comment,
                time_epoch,
            ))
        })
        .collect()
}

// Parses metadata for every session.
pub fn session_data(json: &Value) -> Vec<(u8, String, usize, (i64, i64))> {
    let props = json.get("properties").and_then(|p| p.get("sessionData"));
    if props.is_none() {
        return vec![];
    }

    let data_str = props.unwrap().as_str().unwrap_or("{}");
    let data: Value = match serde_json::from_str(data_str) {
        Ok(json) => json,
        Err(_) => {
            return vec![];
        }
    };

    let mut session_data = Vec::new();

    if let Some(obj) = data.as_object() {
        for (key, value) in obj.iter() {
            let id: u8 = key.parse().unwrap_or(0);
            let name = value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let rank = value.get("rank").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

            let date1 = value
                .get("date")
                .and_then(|v| v.get(0))
                .and_then(|v| v.as_i64())
                .unwrap_or(-1);
            let date2 = value
                .get("date")
                .and_then(|v| v.get(1))
                .and_then(|v| v.as_i64())
                .unwrap_or(-1);

            session_data.push((id, name, rank, (date1, date2)));
        }
    }

    session_data
}
