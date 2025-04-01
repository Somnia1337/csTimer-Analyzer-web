use std::convert::TryFrom;
use std::fmt;

use crate::errors::{ParseAnalysisError, ParseStatsError};
use crate::types::Milliseconds;

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
pub enum AnalysisOption {
    Summary,
    Pbs(StatsType),
    Group(StatsType, Milliseconds),
    Trend(StatsType),
    Commented,
}

impl fmt::Display for AnalysisOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            AnalysisOption::Summary => String::from("Summary"),
            AnalysisOption::Pbs(stats_type) => format!("PBs({})", stats_type),
            AnalysisOption::Group(stats_type, interval) => {
                format!("Group({}, by {}s)", stats_type, *interval as f32 / 1000.0)
            }
            AnalysisOption::Trend(stats_type) => format!("Trend({})", stats_type),
            AnalysisOption::Commented => String::from("Commented"),
        };

        write!(f, "{}", label)
    }
}

impl TryFrom<&str> for AnalysisOption {
    type Error = ParseAnalysisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().to_lowercase();

        if value == "summary" {
            return Ok(AnalysisOption::Summary);
        }

        if let Some(inner) = value.strip_prefix("pbs(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(AnalysisOption::Pbs(stats));
            }
        }

        if let Some(inner) = value.strip_prefix("group(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let splits: Vec<&str> = inner.split(',').collect();
                let stats = StatsType::try_from(splits[0])?;
                let interval = match splits[1].trim().parse() {
                    Ok(int) => int,
                    Err(e) => {
                        return Err(ParseAnalysisError::InvalidStats(ParseStatsError::from(e)));
                    }
                };
                return Ok(AnalysisOption::Group(stats, interval));
            }
        }

        if let Some(inner) = value.strip_prefix("trend(") {
            if let Some(inner) = inner.strip_suffix(")") {
                let stats = StatsType::try_from(inner)?;
                return Ok(AnalysisOption::Trend(stats));
            }
        }

        if value == "commented" {
            return Ok(AnalysisOption::Commented);
        }

        Err(ParseAnalysisError::InvalidFormat)
    }
}

/// Ignores blank lines and removes
/// comments from options file.
fn sanitize_options(options: &str) -> Vec<String> {
    options
        .lines()
        .map(|op| {
            let end = op.to_string().find('#').unwrap_or(op.len());
            op[0..end].trim().to_string()
        })
        .filter(|op| !op.is_empty())
        .collect()
}

/// Parses options and removes duplicates.
pub fn parse_options(options: &str) -> Vec<AnalysisOption> {
    // Sanitizes options
    let options = sanitize_options(options);

    // Removes duplicates
    let mut seen = std::collections::HashSet::new();
    let options: Vec<AnalysisOption> = options
        .into_iter()
        .filter(|s| seen.insert(s.clone()))
        .filter_map(|s| AnalysisOption::try_from(s.as_str()).ok())
        .collect();

    options
}
