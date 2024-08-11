use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
};

pub type CollectResult<T> = std::result::Result<T, MetricError>;

/// An error received from sysmetrics
#[derive(Debug)]
pub enum MetricError {
    /// Platform does not support Desktop Management Interface (DMI) information
    DmiSupportError,

    /// IO read error
    IOError(PathBuf, std::io::Error),

    /// json serde pretty error
    SerdeJsonError(serde_json::Error),

    /// parse int error
    ParseIntError(String, ParseIntError),

    /// parse int error
    ParseFloatError(String, ParseFloatError),

    /// Byte convert error
    ByteConvertError(String),

    /// Invalid fields number
    InvalidFieldNumberError(String, usize, String),

    /// Process not found
    ProcessNotFound(usize),
}

impl fmt::Display for MetricError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricError::DmiSupportError => write!(
                f,
                "platform does not support Desktop Management Interface (DMI) information",
            ),
            MetricError::IOError(ref p, ref e) => {
                write!(f, "cannot read sysfs {:?}: {}", p, e)
            }
            MetricError::SerdeJsonError(ref e) => write!(f, "json pretty error: {}", e),
            MetricError::ParseIntError(ref item, ref e) => {
                write!(f, "{} parse {} int error", item, e)
            }
            MetricError::ParseFloatError(ref item, ref e) => {
                write!(f, "{} parse {} float error", item, e)
            }
            MetricError::ProcessNotFound(ref pid) => write!(f, "process (pid={}) not found", pid),
            MetricError::ByteConvertError(ref unit) => write!(f, "invalid unit: {}", unit),
            MetricError::InvalidFieldNumberError(ref title, ref num, ref fields) => {
                write!(f, "invalid {} fields number {}: {:?}", title, num, fields)
            }
        }
    }
}
