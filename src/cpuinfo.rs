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
#[derive(Debug, Serialize, Clone)]
pub struct CpuInfo {
    pub processor: u32,
    pub vendor_id: Option<String>,
    pub cpu_family: Option<u32>,
    pub model: Option<u32>,
    pub model_name: Option<String>,
    pub stepping: Option<u32>,
    pub microcode: Option<String>,
    pub cpu_mhz: Option<f64>,
    pub cache_size_bytes: Option<u64>,
    pub physical_id: Option<u32>,
    pub siblings: Option<u32>,
    pub core_id: Option<u32>,
    pub cpu_cores: Option<u32>,
    pub apic_id: Option<u32>,
    pub initial_apic_id: Option<u32>,
    pub fpu: Option<String>,
    pub fpu_exception: Option<String>,
    pub cpu_id_level: Option<u32>,
    pub wp: Option<String>,
    pub flags: Vec<String>,
    pub vmx_flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: Option<f64>,
    pub clflush_size: Option<u32>,
    pub cache_alignment: Option<u32>,
    pub address_sizes: Option<String>,
    pub power_management: Option<String>,
}

impl CpuInfo {
    fn new(processor: u32) -> Self {
        Self {
            processor,
            vendor_id: None,
            cpu_family: None,
            model: None,
            model_name: None,
            stepping: None,
            microcode: None,
            cpu_mhz: None,
            cache_size_bytes: None,
            physical_id: None,
            siblings: None,
            core_id: None,
            cpu_cores: None,
            apic_id: None,
            initial_apic_id: None,
            fpu: None,
            fpu_exception: None,
            cpu_id_level: None,
            wp: None,
            flags: Vec::new(),
            vmx_flags: Vec::new(),
            bugs: Vec::new(),
            bogomips: None,
            clflush_size: None,
            cache_alignment: None,
            address_sizes: None,
            power_management: None,
        }
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
                sys_cpuinfo.push(CpuInfo::new(processor));
            }
            CpuInfoInfo::VendorID => sys_cpuinfo[info_index].vendor_id = metric_value,
            CpuInfoInfo::CpuFamily => {
                sys_cpuinfo[info_index].cpu_family =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu family: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::Model => {
                sys_cpuinfo[info_index].model =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu model: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::ModelName => sys_cpuinfo[info_index].model_name = metric_value,
            CpuInfoInfo::Stepping => {
                sys_cpuinfo[info_index].stepping =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu stepping: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::Microcode => sys_cpuinfo[info_index].microcode = metric_value,
            CpuInfoInfo::CpuMhz => {
                sys_cpuinfo[info_index].cpu_mhz =
                    match metric_value.unwrap_or_default().parse::<f64>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu mhz: {:?}", err);
                            None
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
                    utils::convert_to_bytes(item_value, item_unit);
            }
            CpuInfoInfo::PhysicalID => {
                sys_cpuinfo[info_index].physical_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu physical id: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::Siblings => {
                sys_cpuinfo[info_index].siblings =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu siblings: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::CoreID => {
                sys_cpuinfo[info_index].core_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu core id: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::CpuCores => {
                sys_cpuinfo[info_index].cpu_cores =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu cores: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::ApicID => {
                sys_cpuinfo[info_index].apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu apic id: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::InitialApicID => {
                sys_cpuinfo[info_index].initial_apic_id =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu initial apic id: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::Fpu => sys_cpuinfo[info_index].fpu = metric_value,
            CpuInfoInfo::FpuException => sys_cpuinfo[info_index].fpu_exception = metric_value,
            CpuInfoInfo::CpuIDLevel => {
                sys_cpuinfo[info_index].cpu_id_level =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu id level: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::Wp => sys_cpuinfo[info_index].wp = metric_value,
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
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu bogomips: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::ClflushSize => {
                sys_cpuinfo[info_index].clflush_size =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu clflush size: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::CacheAlignment => {
                sys_cpuinfo[info_index].cache_alignment =
                    match metric_value.unwrap_or_default().parse::<u32>() {
                        Ok(c) => Some(c),
                        Err(err) => {
                            log::error!("failed to parse cpu cache alignment: {:?}", err);
                            None
                        }
                    };
            }
            CpuInfoInfo::AddressSize => sys_cpuinfo[info_index].address_sizes = metric_value,
            CpuInfoInfo::PowerManagement => {
                sys_cpuinfo[info_index].power_management = metric_value;
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
