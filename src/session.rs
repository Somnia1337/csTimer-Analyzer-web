use std::fmt;
use std::rc::Rc;

use chrono::DateTime;

use crate::record::Record;
use crate::time::Milliseconds;

/// A training session, same as the "session" in csTimer.
#[derive(Debug, Clone)]
pub struct Session {
    rank: usize,
    name: String,
    date_time: (i64, i64),
    records: Vec<Rc<Record>>,
    records_not_dnf: Vec<Rc<Record>>,
}

impl Session {
    /// Creates a new `Session` from its fields.
    pub fn from(rank: usize, name: String, date_time: (i64, i64), records: &[Record]) -> Self {
        let records: Vec<Rc<Record>> = records.iter().cloned().map(Rc::new).collect();
        let records_not_dnf = records
            .iter()
            .filter(|r| !r.solve_state().is_dnf())
            .cloned()
            .collect();

        Self {
            rank,
            name,
            date_time,
            records,
            records_not_dnf,
        }
    }

    /// The name of a `Session`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The rank of a `Session`.
    pub fn rank(&self) -> usize {
        self.rank
    }

    /// The start and end date-times of a `Session`,
    /// both in `chrono::DateTime`.
    pub fn date_time(&self) -> (DateTime<chrono::Utc>, DateTime<chrono::Utc>) {
        (
            DateTime::from_timestamp(self.date_time.0, 0).unwrap_or_default(),
            DateTime::from_timestamp(self.date_time.1, 0).unwrap_or_default(),
        )
    }

    /// The records of a `Session`.
    pub fn records(&self) -> &[Rc<Record>] {
        &self.records
    }

    /// The records of a `Session` that are not DNF.
    pub fn records_not_dnf(&self) -> &[Rc<Record>] {
        &self.records_not_dnf
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[#{}] **{}** (`{}` records)",
            self.rank(),
            self.name(),
            self.records().len(),
        )
    }
}

/// A group of records, by a time interval.
pub type GroupTime = (Milliseconds, usize);
