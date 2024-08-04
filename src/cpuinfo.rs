use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

enum CpuInfoInfo {
    Processor,
    VendorID,
    CpuFamily,
    Model,
    ModelName,
    Stepping,
    Microcode,
    CpuMhz,
    CacheSizeBytes,
    PhysicalID,
    Siblings,
    CoreID,
    CpuCores,
    ApicID,
    InitialApicID,
    Fpu,
    FpuException,
    CpuIDLevel,
    Wp,
    Flags,
    VmxFlags,
    Bugs,
    BogoMips,
    ClflushSize,
    CacheAlignment,
    AddressSize,
    PowerManagement,
    Unknown,
}

impl CpuInfoInfo {
    fn from(name: &str) -> CpuInfoInfo {
        match name {
            "processor" => CpuInfoInfo::Processor,
            "vendor_id" => CpuInfoInfo::VendorID,
            "cpu family" => CpuInfoInfo::CpuFamily,
            "model" => CpuInfoInfo::Model,
            "model name" => CpuInfoInfo::ModelName,
            "stepping" => CpuInfoInfo::Stepping,
            "microcode" => CpuInfoInfo::Microcode,
            "cpu MHz" => CpuInfoInfo::CpuMhz,
            "cache size" => CpuInfoInfo::CacheSizeBytes,
            "physical id" => CpuInfoInfo::PhysicalID,
            "siblings" => CpuInfoInfo::Siblings,
            "core id" => CpuInfoInfo::CoreID,
            "cpu cores" => CpuInfoInfo::CpuCores,
            "apicid" => CpuInfoInfo::ApicID,
            "initial apicid" => CpuInfoInfo::InitialApicID,
            "fpu" => CpuInfoInfo::Fpu,
            "fpu_exception" => CpuInfoInfo::FpuException,
            "cpuid level" => CpuInfoInfo::CpuIDLevel,
            "wp" => CpuInfoInfo::Wp,
            "flags" => CpuInfoInfo::Flags,
            "vmx flags" => CpuInfoInfo::VmxFlags,
            "bugs" => CpuInfoInfo::Bugs,
            "bogomips" => CpuInfoInfo::BogoMips,
            "clflush size" => CpuInfoInfo::ClflushSize,
            "cache_alignment" => CpuInfoInfo::CacheAlignment,
            "address sizes" => CpuInfoInfo::AddressSize,
            "power management" => CpuInfoInfo::PowerManagement,
            _ => CpuInfoInfo::Unknown,
        }
    }
}

/// CpuInfo contains general information about a system CPU found in /proc/cpuinfo
#[derive(Debug, Serialize, Clone, Default)]
pub struct CpuInfo {
    pub processor: u32,
    pub vendor_id: String,
    pub cpu_family: u32,
    pub model: u32,
    pub model_name: String,
    pub stepping: u32,
    pub microcode: String,
    pub cpu_mhz: f64,
    pub cache_size_bytes: u64,
    pub physical_id: u32,
    pub siblings: u32,
    pub core_id: u32,
    pub cpu_cores: u32,
    pub apic_id: u32,
    pub initial_apic_id: u32,
    pub fpu: String,
    pub fpu_exception: String,
    pub cpu_id_level: u32,
    pub wp: String,
    pub flags: Vec<String>,
    pub vmx_flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: f64,
    pub clflush_size: u32,
    pub cache_alignment: u32,
    pub address_sizes: String,
    pub power_management: String,
}

impl CpuInfo {
    fn new() -> Self {
        Default::default()
    }
}

/// collects information about current system CPUs
/// # Example
/// ```
/// use procsys::cpuinfo;
///
/// let sys_cpuinfo = cpuinfo::collect().expect("cpu information");
/// let json_output = serde_json::to_string_pretty(&sys_cpuinfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<CpuInfo>> {
    collect_from("/proc/cpuinfo")
}

fn collect_from(filename: &str) -> CollectResult<Vec<CpuInfo>> {
    let mut sys_cpuinfo: Vec<CpuInfo> = Vec::new();

    let mut info_index = 0;
    for line in utils::read_file_lines(filename)? {
        if line.trim().is_empty() {
            continue;
        }

        let item_fields: Vec<&str> = line.trim().split(':').filter(|s| !s.is_empty()).collect();

        let metric = item_fields[0].trim();
        let mut metric_value = None;

        if item_fields.len() == 2 {
            metric_value = Some(item_fields[1].trim().to_string());
        }

        if metric_value.is_none() {
            continue;
        }

        match CpuInfoInfo::from(metric) {
            CpuInfoInfo::Processor => {
                if !sys_cpuinfo.is_empty() {
                    info_index += 1;
                }

                let processor = metric_value
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or_default();

                let mut cpuinfo = CpuInfo::new();
                cpuinfo.processor = processor;
                sys_cpuinfo.push(cpuinfo);
            }
            CpuInfoInfo::VendorID => {
                sys_cpuinfo[info_index].vendor_id = metric_value.unwrap_or_default()
            }
            CpuInfoInfo::CpuFamily => {
                sys_cpuinfo[info_index].cpu_family =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError("cpu family".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::Model => {
                sys_cpuinfo[info_index].model =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError("cpu model".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::ModelName => {
                sys_cpuinfo[info_index].model_name = metric_value.unwrap_or_default()
            }
            CpuInfoInfo::Stepping => {
                sys_cpuinfo[info_index].stepping = match metric_value
                    .unwrap_or_default()
                    .parse::<u32>()
                {
                    Ok(c) => c,
                    Err(err) => {
                        return Err(MetricError::ParseIntError("cpu stepping".to_string(), err));
                    }
                };
            }
            CpuInfoInfo::Microcode => {
                sys_cpuinfo[info_index].microcode = metric_value.unwrap_or_default()
            }
            CpuInfoInfo::CpuMhz => {
                sys_cpuinfo[info_index].cpu_mhz =
                    match metric_value.unwrap_or_default().parse::<f64>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseFloatError("cpu mhz".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::CacheSizeBytes => {
                let value = metric_value.unwrap_or_default();
                let value_fields: Vec<&str> =
                    value.trim().split(' ').filter(|s| !s.is_empty()).collect();

                let item_value = value_fields[0].parse::<u64>().unwrap_or_default();
                let mut item_unit = "B";
                if value_fields.len() == 2 {
                    item_unit = value_fields[1];
                }

                sys_cpuinfo[info_index].cache_size_bytes =
                    utils::convert_to_bytes(item_value, item_unit)?.unwrap();
            }
            CpuInfoInfo::PhysicalID => {
                sys_cpuinfo[info_index].physical_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError(
                                "cpu physical id".to_string(),
                                err,
                            ));
                        }
                    };
            }
            CpuInfoInfo::Siblings => {
                sys_cpuinfo[info_index].siblings = match metric_value
                    .unwrap_or_default()
                    .parse::<u32>()
                {
                    Ok(c) => c,
                    Err(err) => {
                        return Err(MetricError::ParseIntError("cpu siblings".to_string(), err));
                    }
                };
            }
            CpuInfoInfo::CoreID => {
                sys_cpuinfo[info_index].core_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError("cpu core id".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::CpuCores => {
                sys_cpuinfo[info_index].cpu_cores =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError("cpu cores".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::ApicID => {
                sys_cpuinfo[info_index].apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError("cpu apic id".to_string(), err));
                        }
                    };
            }
            CpuInfoInfo::InitialApicID => {
                sys_cpuinfo[info_index].initial_apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError(
                                "cpu initial apic id".to_string(),
                                err,
                            ));
                        }
                    };
            }
            CpuInfoInfo::Fpu => sys_cpuinfo[info_index].fpu = metric_value.unwrap_or_default(),
            CpuInfoInfo::FpuException => {
                sys_cpuinfo[info_index].fpu_exception = metric_value.unwrap_or_default();
            }
            CpuInfoInfo::CpuIDLevel => {
                sys_cpuinfo[info_index].cpu_id_level = match metric_value
                    .unwrap_or_default()
                    .parse::<u32>()
                {
                    Ok(c) => c,
                    Err(err) => {
                        return Err(MetricError::ParseIntError("cpu id level".to_string(), err));
                    }
                };
            }
            CpuInfoInfo::Wp => sys_cpuinfo[info_index].wp = metric_value.unwrap_or_default(),
            CpuInfoInfo::Flags => {
                for flag in metric_value.unwrap_or_default().trim().split(' ') {
                    sys_cpuinfo[info_index].flags.push(flag.to_string());
                }
            }
            CpuInfoInfo::VmxFlags => {
                for flag in metric_value.unwrap_or_default().trim().split(' ') {
                    sys_cpuinfo[info_index].vmx_flags.push(flag.to_string());
                }
            }
            CpuInfoInfo::Bugs => {
                for flag in metric_value.unwrap_or_default().trim().split(' ') {
                    sys_cpuinfo[info_index].bugs.push(flag.to_string());
                }
            }
            CpuInfoInfo::BogoMips => {
                sys_cpuinfo[info_index].bogomips =
                    match metric_value.unwrap_or_default().parse::<f64>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseFloatError(
                                "cpu bogomips".to_string(),
                                err,
                            ));
                        }
                    };
            }
            CpuInfoInfo::ClflushSize => {
                sys_cpuinfo[info_index].clflush_size =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError(
                                "cpu clflush size".to_string(),
                                err,
                            ));
                        }
                    };
            }
            CpuInfoInfo::CacheAlignment => {
                sys_cpuinfo[info_index].cache_alignment =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            return Err(MetricError::ParseIntError(
                                "cpu cache alignment".to_string(),
                                err,
                            ));
                        }
                    };
            }
            CpuInfoInfo::AddressSize => {
                sys_cpuinfo[info_index].address_sizes = metric_value.unwrap_or_default();
            }
            CpuInfoInfo::PowerManagement => {
                sys_cpuinfo[info_index].power_management = metric_value.unwrap_or_default();
            }
            CpuInfoInfo::Unknown => {}
        }
    }

    Ok(sys_cpuinfo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpuinfo() {
        let sys_cpuinfo =
            collect_from("test_data/fixtures/proc/cpuinfo").expect("collecting cpu information");
        assert_eq!(sys_cpuinfo.len(), 2);

        for cpu in sys_cpuinfo {
            match cpu.processor {
                0 => {
                    assert_eq!(cpu.vendor_id, "GenuineIntel");
                    assert_eq!(cpu.cpu_family, 6);
                    assert_eq!(cpu.model, 142);
                    assert_eq!(cpu.model_name, "Intel(R) Core(TM) i7-8650U CPU @ 1.90GHz");
                    assert_eq!(cpu.stepping, 10);
                    assert_eq!(cpu.microcode, "0xb4");
                    assert_eq!(cpu.cpu_mhz, 799.998);
                    assert_eq!(cpu.cache_size_bytes, 8388608);
                    assert_eq!(cpu.physical_id, 0);
                    assert_eq!(cpu.siblings, 8);
                    assert_eq!(cpu.core_id, 0);
                    assert_eq!(cpu.cpu_cores, 4);
                    assert_eq!(cpu.apic_id, 0);
                    assert_eq!(cpu.initial_apic_id, 0);
                    assert_eq!(cpu.fpu, "yes");
                    assert_eq!(cpu.fpu_exception, "yes");
                    assert_eq!(cpu.cpu_id_level, 22);
                    assert_eq!(cpu.wp, "yes");
                    assert!(cpu.vmx_flags.is_empty());
                    assert_eq!(
                        cpu.bugs,
                        [
                            "cpu_meltdown",
                            "spectre_v1",
                            "spectre_v2",
                            "spec_store_bypass",
                            "l1tf",
                            "mds",
                            "swapgs",
                        ]
                    );
                    assert_eq!(cpu.clflush_size, 64);
                    assert_eq!(cpu.cache_alignment, 64);
                    assert_eq!(cpu.address_sizes, "39 bits physical, 48 bits virtual");
                    assert_eq!(cpu.bogomips, 4224.00);
                    assert!(cpu.power_management.is_empty());
                }
                1 => {
                    assert_eq!(cpu.vendor_id, "GenuineIntel");
                    assert_eq!(cpu.cpu_family, 6);
                    assert_eq!(cpu.model, 142);
                    assert_eq!(cpu.model_name, "Intel(R) Core(TM) i7-8650U CPU @ 1.90GHz");
                    assert_eq!(cpu.stepping, 10);
                    assert_eq!(cpu.microcode, "0xb4");
                    assert_eq!(cpu.cpu_mhz, 800.037);
                    assert_eq!(cpu.cache_size_bytes, 8388608);
                    assert_eq!(cpu.physical_id, 0);
                    assert_eq!(cpu.siblings, 8);
                    assert_eq!(cpu.core_id, 1);
                    assert_eq!(cpu.cpu_cores, 4);
                    assert_eq!(cpu.apic_id, 2);
                    assert_eq!(cpu.initial_apic_id, 2);
                    assert_eq!(cpu.fpu, "yes");
                    assert_eq!(cpu.fpu_exception, "yes");
                    assert_eq!(cpu.cpu_id_level, 22);
                    assert_eq!(cpu.wp, "yes");
                    assert!(cpu.vmx_flags.is_empty());
                    assert_eq!(
                        cpu.bugs,
                        [
                            "cpu_meltdown",
                            "spectre_v1",
                            "spectre_v2",
                            "spec_store_bypass",
                            "l1tf",
                            "mds",
                            "swapgs",
                        ]
                    );
                    assert_eq!(cpu.clflush_size, 64);
                    assert_eq!(cpu.cache_alignment, 64);
                    assert_eq!(cpu.address_sizes, "39 bits physical, 48 bits virtual");
                    assert_eq!(cpu.bogomips, 4224.00);
                    assert!(cpu.power_management.is_empty());
                }
                _ => panic!("invalid processor: {}", cpu.processor),
            }
        }
    }
}
