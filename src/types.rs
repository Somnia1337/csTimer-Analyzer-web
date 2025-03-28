use std::convert::TryFrom;
use std::{fmt, rc::Rc};

use crate::errors::*;
use crate::record::*;

/// Milliseconds in u32.
pub type Milliseconds = u32;

/// Seconds in f32.
pub type Seconds = f32;

/// Milliseconds in a second.
const SEC: Milliseconds = 1000;

/// Milliseconds in a minute.
const MIN: Milliseconds = 60000;

/// Milliseconds in an hour.
const HOUR: Milliseconds = 3600000;

/// Formats a time type into a
/// human-readable string.
pub trait TimeReadable {
    fn readable(&self) -> String;
}

impl TimeReadable for Milliseconds {
    fn readable(&self) -> String {
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
    fn readable(&self) -> String {
        let millis = (*self * 1000.0) as Milliseconds;

        millis.readable()
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

/// The scale(number) of statistics.
type StatsScale = usize;

/// The type of statistics.
#[derive(Clone, Copy)]
pub enum StatsType {
    /// A single solve.
    Single,

    /// The plain-mean of some solves, no DNF allowed.
    Mean(StatsScale),

    /// The cutoff average of some solves,
    /// cutting off at least 5% records at both ends,
    /// up to 5% DNFs are allowed.
    Average(StatsScale),
}

impl fmt::Display for StatsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            StatsType::Single => String::from("Single"),
            StatsType::Mean(scale) => format!("mo{}", scale),
            StatsType::Average(scale) => format!("ao{}", scale),
        };

        write!(f, "{}", label)
    }
}

impl TryFrom<&str> for StatsType {
    type Error = ParseStatsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().to_lowercase();

        if value == "single" {
            return Ok(StatsType::Single);
        }

        if let Some(inner) = value.strip_prefix("mo") {
            let scale = inner.parse::<StatsScale>()?;

            return if scale > 0 {
                Ok(StatsType::Mean(scale))
            } else {
                Err(ParseStatsError::ScaleIsZero)
            };
        }

        if let Some(inner) = value.strip_prefix("ao") {
            let scale = inner.parse::<StatsScale>()?;

            return if scale > 0 {
                Ok(StatsType::Average(scale))
            } else {
                Err(ParseStatsError::ScaleIsZero)
            };
        }

        Err(ParseStatsError::InvalidFormat)
    }
}

/// Option of a single analysis.
pub enum Analyze {
    Overview,
    PbHistory(StatsType),
    Grouping(StatsType, Milliseconds),
    Trending(StatsType),
    Commented,
}

impl fmt::Display for Analyze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Analyze::Overview => String::from("Overview"),
            Analyze::PbHistory(stats_type) => format!("PbHistory({})", stats_type),
            Analyze::Grouping(stats_type, interval) => {
                format!(
                    "Grouping({}, by {}s)",
                    stats_type,
                    *interval as f32 / 1000.0
                )
            }
            Analyze::Trending(stats_type) => format!("Trending({})", stats_type),
            Analyze::Commented => String::from("Commented"),
        };

        write!(f, "{}", label)
    }
}

impl TryFrom<&str> for Analyze {
    type Error = ParseAnalysisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim().to_lowercase();
        if let Some(i) = value.find("#") {
            value = value[0..i].to_string();
        }

        if value == "overview" {
            return Ok(Analyze::Overview);
        }

        if let Some(inner) = value.strip_prefix("pbhistory(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(Analyze::PbHistory(stats));
            }
        }

        if let Some(inner) = value.strip_prefix("grouping(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let splits: Vec<&str> = inner.split(",").collect();
                let stats = StatsType::try_from(splits[0])?;
                let interval = match splits[1].trim().parse() {
                    Ok(int) => int,
                    Err(e) => {
                        return Err(ParseAnalysisError::InvalidStats(ParseStatsError::from(e)));
                    }
                };
                return Ok(Analyze::Grouping(stats, interval));
            }
        }

        if let Some(inner) = value.strip_prefix("trending(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(Analyze::Trending(stats));
            }
        }

        if value == "commented" {
            return Ok(Analyze::Commented);
        }

        Err(ParseAnalysisError::InvalidFormat)
    }
}
