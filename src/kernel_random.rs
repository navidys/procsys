use std::path::Path;

use serde::Serialize;

use crate::{error::CollectResult, utils};

enum KernelRandomInfo {
    EntropyAvaliable,
    PoolSize,
    URandomMinReseedSeconds,
    WriteWakeupThreshold,
    ReadWakeupThreshold,
    Unknown,
}

impl KernelRandomInfo {
    fn from(name: &str) -> KernelRandomInfo {
        match name {
            "entropy_avail" => KernelRandomInfo::EntropyAvaliable,
            "poolsize" => KernelRandomInfo::PoolSize,
            "urandom_min_reseed_secs" => KernelRandomInfo::URandomMinReseedSeconds,
            "write_wakeup_threshold" => KernelRandomInfo::WriteWakeupThreshold,
            "read_wakeup_threshold" => KernelRandomInfo::ReadWakeupThreshold,
            _ => KernelRandomInfo::Unknown,
        }
    }
}

/// KernelRandom contains information about to the kernel's random number generator
#[derive(Debug, Serialize, Clone, Default)]
pub struct KernelRandom {
    pub entropy_available: Option<u64>,
    pub pool_size: Option<u64>,
    pub urandom_min_reseed_secs: Option<u64>,
    pub write_wakeup_threshold: Option<u64>,
    pub read_wakeup_threshold: Option<u64>,
}

impl KernelRandom {
    fn new() -> Self {
        Default::default()
    }
}

/// collects and return kernel's random number generator information
/// from /proc/sys/kernel/random
/// # Example
/// ```
/// use procsys::kernel_random;
///
/// let krandom = kernel_random::collect().expect("kernel random generator");
///
/// let json_output = serde_json::to_string_pretty(&krandom).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<KernelRandom> {
    let mut krandom = KernelRandom::new();
    let krandom_dir = Path::new("/proc/sys/kernel/random");

    for item in utils::list_dir_content(krandom_dir, "", "random") {
        match KernelRandomInfo::from(&item) {
            KernelRandomInfo::EntropyAvaliable => {
                krandom.entropy_available = utils::collect_info_u64(&item, krandom_dir)?
            }
            KernelRandomInfo::PoolSize => {
                krandom.pool_size = utils::collect_info_u64(&item, krandom_dir)?
            }
            KernelRandomInfo::URandomMinReseedSeconds => {
                krandom.urandom_min_reseed_secs = utils::collect_info_u64(&item, krandom_dir)?
            }
            KernelRandomInfo::WriteWakeupThreshold => {
                krandom.write_wakeup_threshold = utils::collect_info_u64(&item, krandom_dir)?
            }
            KernelRandomInfo::ReadWakeupThreshold => {
                krandom.read_wakeup_threshold = utils::collect_info_u64(&item, krandom_dir)?
            }
            KernelRandomInfo::Unknown => {}
        }
    }

    Ok(krandom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_random_gen() {
        let min_value: u64 = 0;
        let krandom = collect().expect("collecting random number generator information");
        assert!(krandom.entropy_available.unwrap_or_default().ge(&min_value));
        assert!(krandom.pool_size.unwrap_or_default().ge(&min_value));
        assert!(krandom
            .urandom_min_reseed_secs
            .unwrap_or_default()
            .ge(&min_value));
        assert!(krandom
            .write_wakeup_threshold
            .unwrap_or_default()
            .ge(&min_value));
        assert!(krandom
            .read_wakeup_threshold
            .unwrap_or_default()
            .ge(&min_value));
    }
}
