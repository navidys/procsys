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
    let krandom_dir = Path::new("/proc/sys/kernel/random");
    collect_from(krandom_dir)
}

fn collect_from(base_path: &Path) -> CollectResult<KernelRandom> {
    let mut krandom = KernelRandom::new();

    for item in utils::list_dir_content(base_path, "", "random") {
        match KernelRandomInfo::from(&item) {
            KernelRandomInfo::EntropyAvaliable => {
                krandom.entropy_available = utils::collect_info_u64(&item, base_path)?
            }
            KernelRandomInfo::PoolSize => {
                krandom.pool_size = utils::collect_info_u64(&item, base_path)?
            }
            KernelRandomInfo::URandomMinReseedSeconds => {
                krandom.urandom_min_reseed_secs = utils::collect_info_u64(&item, base_path)?
            }
            KernelRandomInfo::WriteWakeupThreshold => {
                krandom.write_wakeup_threshold = utils::collect_info_u64(&item, base_path)?
            }
            KernelRandomInfo::ReadWakeupThreshold => {
                krandom.read_wakeup_threshold = utils::collect_info_u64(&item, base_path)?
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
        let krandom_dir = Path::new("test_data/fixtures/proc/sys/kernel/random");
        let krandom =
            collect_from(krandom_dir).expect("collecting random number generator information");

        assert_eq!(krandom.entropy_available.unwrap(), 3943);
        assert_eq!(krandom.pool_size.unwrap(), 4096);
        assert_eq!(krandom.urandom_min_reseed_secs.unwrap(), 60);
        assert_eq!(krandom.write_wakeup_threshold.unwrap(), 3072);
        assert!(krandom.read_wakeup_threshold.is_none());
    }
}
