use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::num::ParseIntError;

use crate::time::{Milliseconds, ToSeconds};

/// The scale of statistics.
type StatsScale = usize;

/// The type of statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatsType {
    /// A single solve.
    Single,

    /// The plain-mean of some solves, no DNF allowed.
    Mean(StatsScale),

    /// The cutoff average of some solves,
    /// cutting off at least 5% records on both ends,
    /// up to 5% DNFs are allowed.
    Average(StatsScale),
}

impl fmt::Display for StatsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Single => String::from("single"),
            Self::Mean(scale) => format!("mo{}", scale),
            Self::Average(scale) => format!("ao{}", scale),
        };

        write!(f, "{}", label)
    }
}

impl TryFrom<&str> for StatsType {
    type Error = ParseStatsTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().to_lowercase();

        if value == "single" {
            return Ok(Self::Single);
        }

        if let Some(inner) = value.strip_prefix("mo") {
            let scale = inner.parse::<StatsScale>()?;

            return match scale.cmp(&1) {
                Ordering::Less => Err(ParseStatsTypeError::ScaleIsZero),
                Ordering::Equal => Ok(Self::Single),
                Ordering::Greater => Ok(Self::Mean(scale)),
            };
        }

        if let Some(inner) = value.strip_prefix("ao") {
            let scale = inner.parse::<StatsScale>()?;

            return match scale.cmp(&1) {
                Ordering::Less => Err(ParseStatsTypeError::ScaleIsZero),
                Ordering::Equal => Ok(Self::Single),
                Ordering::Greater => Ok(Self::Average(scale)),
            };
        }

        Err(ParseStatsTypeError::InvalidFormat)
    }
}

impl StatsType {
    /// Returns the scale of the stats type.
    pub fn scale(&self) -> StatsScale {
        match self {
            Self::Single => 1,
            Self::Average(scale) | Self::Mean(scale) => *scale,
        }
    }
}

/// An error which can be returned
/// when parsing a `StatsType`.
pub enum ParseStatsTypeError {
    /// Unknown format.
    InvalidFormat,

    /// Parsing integer failed.
    InvalidScale(ParseIntError),

    /// Stats scale is 0.
    ScaleIsZero,
}

impl From<ParseIntError> for ParseStatsTypeError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidScale(err)
    }
}

impl fmt::Display for ParseStatsTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            Self::InvalidFormat => String::from("invalid stats format"),
            Self::InvalidScale(err) => format!("failed to parse int: {}", err),
            Self::ScaleIsZero => String::from("scale must be greater than 0"),
        };

        write!(f, "{}", literal)
    }
}

/// Option of a single analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisOption {
    /// A summary over solve times in the session.
    Summary,

    /// PB histories of some stats type.
    Pbs(StatsType),

    /// Groups of solve times of some stats type,
    /// by some time interval between groups.
    Group(StatsType, Milliseconds),

    /// Trends of solve times of some stats type.
    Trend(StatsType),

    /// `Record`s that has a non-empty comment.
    Commented,
}

impl fmt::Display for AnalysisOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Summary => String::from("Summary"),
            Self::Pbs(stats_type) => format!("PBs(**{}**)", stats_type),
            Self::Group(stats_type, interval) => {
                format!("Group(**{}**, by {}s)", stats_type, interval.as_seconds())
            }
            Self::Trend(stats_type) => format!("Trend(**{}**)", stats_type),
            Self::Commented => String::from("Commented"),
        };

        write!(f, "{}", label)
    }
}

impl TryFrom<&str> for AnalysisOption {
    type Error = ParseAnalysisOptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "summary" {
            return Ok(Self::Summary);
        }

        if let Some(inner) = value.strip_prefix("pbs(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(Self::Pbs(stats));
            }
        }

        if let Some(inner) = value.strip_prefix("group(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let splits: Vec<&str> = inner.split(',').collect();
                let stats = StatsType::try_from(splits[0])?;
                let interval = match splits[1].trim().parse() {
                    Ok(int) => int,
                    Err(e) => {
                        return Err(ParseAnalysisOptionError::InvalidStats(
                            ParseStatsTypeError::from(e),
                        ));
                    }
                };
                return Ok(Self::Group(stats, interval));
            }
        }

        if let Some(inner) = value.strip_prefix("trend(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(Self::Trend(stats));
            }
        }

        if value == "commented" {
            return Ok(Self::Commented);
        }

        Err(ParseAnalysisOptionError::InvalidFormat)
    }
}

impl AnalysisOption {
    /// Returns the stats type of the analysis option.
    pub fn stats_type(&self) -> Option<&StatsType> {
        match self {
            Self::Pbs(s_type) | Self::Group(s_type, _) | Self::Trend(s_type) => Some(s_type),
            _ => None,
        }
    }
}

/// An error which can be returned
/// when parsing an analysis option.
pub enum ParseAnalysisOptionError {
    /// Unknown format.
    InvalidFormat,

    /// Parsing stats type failed.
    InvalidStats(ParseStatsTypeError),
}

impl From<ParseStatsTypeError> for ParseAnalysisOptionError {
    fn from(err: ParseStatsTypeError) -> Self {
        Self::InvalidStats(err)
    }
}

impl fmt::Display for ParseAnalysisOptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            Self::InvalidFormat => String::from("invalid format"),
            Self::InvalidStats(e) => format!("invalid stats param: {}", e),
        };

        write!(f, "{}", literal)
    }
}
