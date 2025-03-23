use std::fmt;
use std::num::ParseIntError;

/// An error which can be returned
/// when parsing a `StatsType`.
pub enum ParseStatsError {
    /// Unknown format
    InvalidFormat,

    /// Parsing integer failed
    InvalidScale(ParseIntError),

    /// Scale number is 0
    ScaleIsZero,
}

impl From<ParseIntError> for ParseStatsError {
    fn from(err: ParseIntError) -> Self {
        ParseStatsError::InvalidScale(err)
    }
}

impl fmt::Display for ParseStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            Self::InvalidFormat => String::from("invalid stats format"),
            Self::InvalidScale(err) => format!("failed to parse int: {}", err),
            Self::ScaleIsZero => String::from("scale must be greater than 0"),
        };

        write!(f, "{}", literal)
    }
}

/// An error which can be returned
/// when parsing an analysis.
pub enum ParseAnalysisError {
    InvalidFormat,
    InvalidStats(ParseStatsError),
}

impl From<ParseStatsError> for ParseAnalysisError {
    fn from(err: ParseStatsError) -> Self {
        ParseAnalysisError::InvalidStats(err)
    }
}

impl fmt::Display for ParseAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self {
            Self::InvalidFormat => String::from("invalid format"),
            Self::InvalidStats(e) => format!("invalid stats param: {}", e),
        };

        write!(f, "{}", literal)
    }
}
