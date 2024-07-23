use serde::Serialize;

use crate::utils;

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
/// let sys_cpuinfo = cpuinfo::collect();
/// let json_output = serde_json::to_string_pretty(&sys_cpuinfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> Vec<CpuInfo> {
    let mut sys_cpuinfo: Vec<CpuInfo> = Vec::new();
    let default_u32 = Default::default();
    let default_f64 = Default::default();

    let mut info_index = 0;
    for line in utils::read_file_lines("/proc/cpuinfo") {
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
                            log::error!("failed to parse cpu family: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::Model => {
                sys_cpuinfo[info_index].model =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu model: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::ModelName => {
                sys_cpuinfo[info_index].model_name = metric_value.unwrap_or_default()
            }
            CpuInfoInfo::Stepping => {
                sys_cpuinfo[info_index].stepping =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu stepping: {:?}", err);
                            default_u32
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
                            log::error!("failed to parse cpu mhz: {:?}", err);
                            default_f64
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
                    utils::convert_to_bytes(item_value, item_unit).unwrap_or_default();
            }
            CpuInfoInfo::PhysicalID => {
                sys_cpuinfo[info_index].physical_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu physical id: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::Siblings => {
                sys_cpuinfo[info_index].siblings =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu siblings: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::CoreID => {
                sys_cpuinfo[info_index].core_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu core id: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::CpuCores => {
                sys_cpuinfo[info_index].cpu_cores =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu cores: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::ApicID => {
                sys_cpuinfo[info_index].apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu apic id: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::InitialApicID => {
                sys_cpuinfo[info_index].initial_apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu initial apic id: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::Fpu => sys_cpuinfo[info_index].fpu = metric_value.unwrap_or_default(),
            CpuInfoInfo::FpuException => {
                sys_cpuinfo[info_index].fpu_exception = metric_value.unwrap_or_default();
            }
            CpuInfoInfo::CpuIDLevel => {
                sys_cpuinfo[info_index].cpu_id_level =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu id level: {:?}", err);
                            default_u32
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
                            log::error!("failed to parse cpu bogomips: {:?}", err);
                            default_f64
                        }
                    };
            }
            CpuInfoInfo::ClflushSize => {
                sys_cpuinfo[info_index].clflush_size =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu clflush size: {:?}", err);
                            default_u32
                        }
                    };
            }
            CpuInfoInfo::CacheAlignment => {
                sys_cpuinfo[info_index].cache_alignment =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => c,
                        Err(err) => {
                            log::error!("failed to parse cpu cache alignment: {:?}", err);
                            default_u32
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

    sys_cpuinfo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpuinfo() {
        let sys_cpuinfo = collect();
        assert!(!sys_cpuinfo.is_empty());

        for cpu in sys_cpuinfo {
            assert!(cpu.processor.ge(&0));
        }
    }
}
