use std::{fmt, rc::Rc};

use crate::record::Record;

/// Milliseconds in u32.
pub type Milliseconds = u32;

/// Seconds in f32.
pub type Seconds = f32;

/// Milliseconds in a second.
const SEC: Milliseconds = 1_000;

/// Milliseconds in a minute.
const MIN: Milliseconds = 60_000;

/// Milliseconds in an hour.
const HOUR: Milliseconds = 3_600_000;

/// Formats a time type into a
/// human-readable string.
pub trait TimeReadable {
    fn to_readable_string(&self) -> String;
}

impl TimeReadable for Milliseconds {
    fn to_readable_string(&self) -> String {
        let t = *self;

        if t < SEC {
            return format!("0.{:03}", t);
        } else if t < SEC * 10 {
            return format!("{}.{:03}", t / SEC, t % SEC);
        }

        let secs_and_millis = format!("{:02}.{:03}", (t % MIN) / SEC, t % SEC);

        if t < MIN {
            secs_and_millis
        } else if t < HOUR {
            let mins = t / MIN;

            format!("{}:{}", mins, secs_and_millis)
        } else {
            let hours = t / HOUR;
            let mins = (t % HOUR) / MIN;

            format!("{}:{}:{}", hours, mins, secs_and_millis)
        }
    }
}

impl TimeReadable for Seconds {
    fn to_readable_string(&self) -> String {
        let millis = (*self * 1000.0) as Milliseconds;

        millis.to_readable_string()
    }
}

/// Valid states of a solve,
/// same as the "state" in csTimer.
#[derive(Debug, Clone, PartialEq)]
pub enum SolveState {
    Ok,
    Plus2,
    Dnf,
}

impl fmt::Display for SolveState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            SolveState::Ok => "Ok",
            SolveState::Plus2 => "+2",
            SolveState::Dnf => "DNF",
        };

        write!(f, "{}", literal)
    }
}

impl SolveState {
    /// Returns true if state is Ok.
    pub fn is_ok(&self) -> bool {
        matches!(self, SolveState::Ok)
    }

    /// Returns true if state is Plus2.
    pub fn is_plus2(&self) -> bool {
        matches!(self, SolveState::Plus2)
    }

    /// Returns true if state is Dnf.
    pub fn is_dnf(&self) -> bool {
        matches!(self, SolveState::Dnf)
    }
}

/// A group of records, by a time interval.
pub struct GroupRecord {
    interval: Milliseconds,
    records: Vec<Rc<Record>>,
}

impl GroupRecord {
    pub fn new(interval: Milliseconds, records: &[Rc<Record>]) -> Self {
        GroupRecord {
            interval,
            records: records.to_vec(),
        }
    }

    pub fn interval(&self) -> Milliseconds {
        self.interval
    }

    pub fn records(&self) -> &[Rc<Record>] {
        &self.records
    }
}
