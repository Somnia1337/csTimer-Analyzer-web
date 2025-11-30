use std::fmt;
use std::rc::Rc;

use chrono::DateTime;

use crate::options::TargetRange;
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
    pub fn from(rank: usize, name: String, date_time: (i64, i64), records: Vec<Record>) -> Self {
        let records: Vec<Rc<Record>> = records.into_iter().map(Rc::new).collect();
        let records_not_dnf = records
            .iter()
            .filter(|r| !r.solve_state().is_dnf())
            .map(Rc::clone)
            .collect();

        Self {
            rank,
            name,
            date_time,
            records,
            records_not_dnf,
        }
    }

    /// Creates a `Session` from an existing one,
    /// which records are within the specified range.
    pub fn try_from_target_range(&self, target_range: &TargetRange) -> Option<Self> {
        let records = self.records_in_target_range(target_range);

        if records.is_empty() {
            return None;
        }

        let records_not_dnf = records
            .iter()
            .filter(|r| !r.solve_state().is_dnf())
            .map(Rc::clone)
            .collect();

        Some(Self {
            records,
            records_not_dnf,
            ..self.clone()
        })
    }

    /// The name of a `Session`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The rank of a `Session`.
    pub const fn rank(&self) -> usize {
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

    /// The `Record`s of a `Session`.
    pub fn records(&self) -> &[Rc<Record>] {
        &self.records
    }

    /// The `Record`s of a `Session` that are not DNF.
    pub fn records_not_dnf(&self) -> &[Rc<Record>] {
        &self.records_not_dnf
    }

    /// The number of `Record`s.
    pub const fn record_count(&self) -> usize {
        self.records.len()
    }

    /// The number of `Record`s that are not DNF.
    pub const fn record_not_dnf_count(&self) -> usize {
        self.records_not_dnf.len()
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[#{}] {}", self.rank(), self.name())
    }
}

impl Session {
    /// The `Record`s within the specified `TargetRange`.
    fn records_in_target_range(&self, target_range: &TargetRange) -> Vec<Rc<Record>> {
        match target_range {
            TargetRange::SolvesCount(count) => {
                let take = *count.min(&self.record_count());

                self.records()
                    .iter()
                    .skip(self.record_count() - take)
                    .map(Rc::clone)
                    .collect()
            }
            TargetRange::Percentage(percentage) => {
                let p = *percentage as f32 / 100.0;
                let take = (self.record_count() as f32 * p).ceil() as usize;

                self.records_in_target_range(&TargetRange::SolvesCount(take))
            }
            TargetRange::DateRange(start, end) => self
                .records()
                .iter()
                .filter(|r| {
                    let date = r.date_time().date_naive();
                    date >= *start && end.as_ref().is_none_or(|end| date <= *end)
                })
                .cloned()
                .collect(),
        }
    }
}

/// A group of `Record`s, with a starting time
/// and a count of the `Record`s.
pub type GroupTime = (Milliseconds, usize);
