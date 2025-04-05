use std::fmt;

use chrono::DateTime;

use crate::time::{HumanReadable, Milliseconds};

/// Valid states of a solve, same as the "state" in csTimer.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SolveState {
    /// No penalty.
    Ok,

    /// Plus 2 seconds.
    Plus2,

    /// Did not finish.
    Dnf,
}

impl fmt::Display for SolveState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            Self::Ok => "Ok",
            Self::Plus2 => "+2",
            Self::Dnf => "DNF",
        };

        write!(f, "{}", literal)
    }
}

impl SolveState {
    /// Returns true if state is Ok.
    pub fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns true if state is Plus2.
    pub fn is_plus2(self) -> bool {
        matches!(self, Self::Plus2)
    }

    /// Returns true if state is Dnf.
    pub fn is_dnf(self) -> bool {
        matches!(self, Self::Dnf)
    }
}

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
    /// Creates a new `Record` from its fields.
    pub fn from(
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

    /// The solve state of a `Record`.
    pub fn solve_state(&self) -> SolveState {
        self.solve_state
    }

    /// The time of a `Record`.
    pub fn time(&self) -> Milliseconds {
        self.time
    }

    /// The scramble of a `Record`.
    pub fn scramble(&self) -> &str {
        &self.scramble
    }

    /// The comment of a `Record`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// The date-time of a `Record`, in `chrono::DateTime`.
    pub fn date_time(&self) -> DateTime<chrono::Utc> {
        DateTime::from_timestamp(self.date_time, 0).unwrap_or_default()
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
            format!("- comment: **{}**\n", self.comment)
        };

        write!(
            f,
            "@{}\n\n{}- time: `{}`\n- scramble: *{}*\n{}",
            self.date_time()
                .to_string()
                .strip_suffix(" UTC")
                .unwrap_or_default(),
            solve_state,
            self.time.to_readable_string(),
            self.scramble,
            comment
        )
    }
}
