use std::fmt;

use chrono::DateTime;

use crate::types::*;

/// A cubing record.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    solve_state: SolveState,
    time: Milliseconds,
    scramble: String,
    comment: String,
    date_time: i64,
}

impl Record {
    pub fn new(
        solve_state: SolveState,
        time: Milliseconds,
        scramble: String,
        comment: String,
        date_time: i64,
    ) -> Self {
        Self {
            solve_state,
            time,
            scramble,
            comment,
            date_time,
        }
    }

    pub fn solve_state(&self) -> &SolveState {
        &self.solve_state
    }

    pub fn time(&self) -> Milliseconds {
        self.time
    }

    pub fn scramble(&self) -> &str {
        &self.scramble
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn date_time(&self) -> DateTime<chrono::Utc> {
        DateTime::from_timestamp(self.date_time, 0).expect("time goes backwards")
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let solve_state = if self.solve_state.is_ok() {
            String::new()
        } else {
            format!("- state: **{}**\n", self.solve_state)
        };

        let comment = if self.comment.is_empty() {
            String::new()
        } else {
            format!("- comment: *{}*\n", self.comment)
        };

        write!(
            f,
            "@{}\n\n{}- time: `{}`\n- scramble: **{}**\n{}",
            self.date_time(),
            solve_state,
            self.time.readable(),
            self.scramble,
            comment
        )
    }
}
