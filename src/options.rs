use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::num::ParseIntError;

use chrono::NaiveDate;

use crate::time::{AsSeconds, Milliseconds};

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
            Self::Single => t!("option.single").to_string(),
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
    pub const fn scale(&self) -> StatsScale {
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

pub enum ParseTargetRangeError {
    InvalidFormat,
    InvalidNumber,
    InvalidPercentage,
    InvalidDateCount,
    InvalidDateFormat(chrono::ParseError),
    InvalidDateRange,
}

impl fmt::Display for ParseTargetRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid target range format"),
            Self::InvalidNumber => write!(f, "expected a number greater than 0"),
            Self::InvalidPercentage => write!(f, "percentage must be in range 1 to 99"),
            Self::InvalidDateCount => write!(f, "expected 1 date or 2 dates"),
            Self::InvalidDateFormat(e) => write!(f, "invalid date format: {}", e),
            Self::InvalidDateRange => write!(f, "the start date must be earlier than the end date"),
        }
    }
}

impl From<chrono::ParseError> for ParseTargetRangeError {
    fn from(err: chrono::ParseError) -> Self {
        Self::InvalidDateFormat(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TargetRange {
    /// Count of solves, must be > 0.
    SolvesCount(usize),

    /// Percentage in [1, 99].
    Percentage(u8),

    /// Start from specific date.
    DateRange(NaiveDate, Option<NaiveDate>),
}

impl fmt::Display for TargetRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolvesCount(count) => write!(f, "{}", count),
            Self::Percentage(pct) => write!(f, "{}%", pct),
            Self::DateRange(start, end) => write!(
                f,
                "{} ~ {}",
                start.format("%Y-%m-%d"),
                match end {
                    Some(end) => end.format("%Y-%m-%d").to_string(),
                    None => t!("option.now").to_string(),
                },
            ),
        }
    }
}

impl TryFrom<&str> for TargetRange {
    type Error = ParseTargetRangeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = input.trim();

        if let Some(stripped) = input.strip_suffix('%') {
            let pct = stripped
                .parse::<u8>()
                .map_err(|_| ParseTargetRangeError::InvalidNumber)?;
            return if (1..=99).contains(&pct) {
                Ok(Self::Percentage(pct))
            } else {
                Err(ParseTargetRangeError::InvalidPercentage)
            };
        }

        if let Ok(count) = input.parse::<usize>() {
            return if count >= 1 {
                Ok(Self::SolvesCount(count))
            } else {
                Err(ParseTargetRangeError::InvalidNumber)
            };
        }

        let dates: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

        let start = NaiveDate::parse_from_str(dates[0], "%Y-%m-%d")?;
        if dates.len() == 1 {
            Ok(Self::DateRange(start, None))
        } else if dates.len() == 2 {
            let end = NaiveDate::parse_from_str(dates[1], "%Y-%m-%d")?;

            if start > end {
                Err(ParseTargetRangeError::InvalidDateRange)
            } else {
                Ok(Self::DateRange(start, Some(end)))
            }
        } else {
            Err(ParseTargetRangeError::InvalidDateCount)
        }
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

    /// Some recent solves specified by a number,
    /// percentage or a range of days.
    Recent(TargetRange),

    /// `Record`s that has a non-empty comment.
    Commented,
}

impl fmt::Display for AnalysisOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Summary => t!("option.summary"),
            Self::Pbs(s_type) => t!("option.pbs", s_type = s_type),
            Self::Group(s_type, interval) => {
                if *interval > 0 {
                    t!(
                        "option.group",
                        s_type = s_type,
                        interval = interval.as_seconds(),
                    )
                } else {
                    t!("option.group-by-zero", s_type = s_type)
                }
            }
            Self::Trend(s_type) => t!("option.trend", s_type = s_type),
            Self::Recent(range) => t!("option.recent", range = range),
            Self::Commented => t!("option.commented"),
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

        if let Some(inner) = value.strip_prefix("recent(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let range = TargetRange::try_from(inner)
                    .map_err(ParseAnalysisOptionError::InvalidTarget)?;
                return Ok(Self::Recent(range));
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
    pub const fn stats_type(&self) -> Option<&StatsType> {
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

    /// Parsing target range failed.
    InvalidTarget(ParseTargetRangeError),
}

impl From<ParseStatsTypeError> for ParseAnalysisOptionError {
    fn from(err: ParseStatsTypeError) -> Self {
        Self::InvalidStats(err)
    }
}

impl fmt::Display for ParseAnalysisOptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidStats(e) => write!(f, "invalid stats param: {}", e),
            Self::InvalidTarget(e) => write!(f, "invalid target param: {}", e),
        }
    }
}
