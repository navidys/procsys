use std::{fmt, path::PathBuf};

/// An error received from sysmetrics
#[derive(Debug)]
pub enum MetricError {
    /// Platform does not support Desktop Management Interface (DMI) information
    DmiSupportError,

    /// IO read error
    IOError(PathBuf, std::io::Error),

    /// json serde pretty error
    SerdeJsonError(serde_json::Error),
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
        }
    }
}
