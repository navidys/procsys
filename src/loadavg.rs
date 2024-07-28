use std::path::Path;

use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// LoadAvg represents an entry in /proc/loadavg
#[derive(Debug, Serialize, Clone, Default)]
pub struct LoadAvg {
    pub load1: f64,
    pub load5: f64,
    pub load15: f64,
}

impl LoadAvg {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the load average information
/// # Example
/// ```
/// use procsys::loadavg;
///
/// let sysload = loadavg::collect().expect("load average");
///
/// println!("load average 1 : {}", sysload.load1);
/// println!("load average 5 : {}", sysload.load5);
/// println!("load average 15: {}", sysload.load15);
/// ```
pub fn collect() -> CollectResult<LoadAvg> {
    let mut sysload = LoadAvg::new();

    match utils::collect_info_string("loadavg", Path::new("/proc"))? {
        Some(content) => {
            let avgfields: Vec<&str> = content
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect();

            if avgfields.len() < 3 {
                return Err(MetricError::InvalidFieldNumberError(
                    "load avegrage".to_string(),
                    avgfields.len(),
                    content,
                ));
            }

            sysload.load1 = avgfields[0].parse::<f64>().unwrap_or_default();
            sysload.load5 = avgfields[1].parse::<f64>().unwrap_or_default();
            sysload.load15 = avgfields[2].parse::<f64>().unwrap_or_default();
        }
        None => return Ok(sysload),
    }

    Ok(sysload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sys_loadavg() {
        let sysload = collect().expect("collecting system load average");
        let min_sysload: f64 = 0.0;

        assert!(sysload.load1.ge(&min_sysload));
        assert!(sysload.load5.ge(&min_sysload));
        assert!(sysload.load15.ge(&min_sysload));
    }
}
