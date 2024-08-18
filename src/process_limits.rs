use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

const MAX_LIMIT_SIZE: u64 = 18446744073709551615;

enum ProcessLimitsType {
    CPUTime,
    FileSize,
    DataSize,
    StackSize,
    CoreFileSize,
    ResidentSet,
    Processes,
    OpenFiles,
    LockedMemery,
    AddressSpace,
    FileLocks,
    PendingSignals,
    MsqqueueSize,
    NicePriority,
    RealtimePriority,
    RealtimeTimeout,
    Unknown,
}

impl ProcessLimitsType {
    fn from(name: &str) -> ProcessLimitsType {
        match name {
            "Max cpu time" => ProcessLimitsType::CPUTime,
            "Max file size" => ProcessLimitsType::FileSize,
            "Max data size" => ProcessLimitsType::DataSize,
            "Max stack size" => ProcessLimitsType::StackSize,
            "Max core file size" => ProcessLimitsType::CoreFileSize,
            "Max resident set" => ProcessLimitsType::ResidentSet,
            "Max processes" => ProcessLimitsType::Processes,
            "Max open files" => ProcessLimitsType::OpenFiles,
            "Max locked memory" => ProcessLimitsType::LockedMemery,
            "Max address space" => ProcessLimitsType::AddressSpace,
            "Max file locks" => ProcessLimitsType::FileLocks,
            "Max pending signals" => ProcessLimitsType::PendingSignals,
            "Max msgqueue size" => ProcessLimitsType::MsqqueueSize,
            "Max nice priority" => ProcessLimitsType::NicePriority,
            "Max realtime priority" => ProcessLimitsType::RealtimePriority,
            "Max realtime timeout" => ProcessLimitsType::RealtimeTimeout,
            _ => ProcessLimitsType::Unknown,
        }
    }
}

/// ProcessLimits represents the soft limits for each of the process's resource limits
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessLimits {
    pub cpu_time: u64,
    pub file_size: u64,
    pub data_size: u64,
    pub stack_size: u64,
    pub core_file_size: u64,
    pub resident_set: u64,
    pub processes: u64,
    pub open_files: u64,
    pub locked_memory: u64,
    pub address_space: u64,
    pub file_locks: u64,
    pub pending_signals: u64,
    pub msgqueue_size: u64,
    pub nice_priority: u64,
    pub realtime_priority: u64,
    pub realtime_timeout: u64,
}

impl ProcessLimits {
    fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// limits returns the current soft limits of the process
    pub fn limits(&self) -> CollectResult<ProcessLimits> {
        let mut proc_limits = ProcessLimits::new();
        let proc_limits_path_str = format!("{:?}", self.path());
        let proc_limits_file = format!("{}/limits", proc_limits_path_str.replace("\"", ""));

        let re = match regex::Regex::new(r"(Max \w+\s{0,1}?\w*\s{0,1}\w*)\s{2,}(\w+)\s+(\w+)") {
            Ok(r) => r,
            Err(e) => return Err(MetricError::RegexError(e)),
        };

        for line in utils::read_file_lines(&proc_limits_file)? {
            if line.starts_with("Limit") {
                continue;
            }

            for (_, [item, soft_limit, _]) in re.captures_iter(&line).map(|c| c.extract()) {
                let mut soft_value: u64 = MAX_LIMIT_SIZE;

                if soft_limit != "unlimited" {
                    soft_value = soft_limit.parse::<u64>().unwrap_or_default();
                }

                match ProcessLimitsType::from(item.trim()) {
                    ProcessLimitsType::CPUTime => proc_limits.cpu_time = soft_value,
                    ProcessLimitsType::FileSize => proc_limits.file_size = soft_value,
                    ProcessLimitsType::DataSize => proc_limits.data_size = soft_value,
                    ProcessLimitsType::StackSize => proc_limits.stack_size = soft_value,
                    ProcessLimitsType::CoreFileSize => proc_limits.core_file_size = soft_value,
                    ProcessLimitsType::ResidentSet => proc_limits.resident_set = soft_value,
                    ProcessLimitsType::Processes => proc_limits.processes = soft_value,
                    ProcessLimitsType::OpenFiles => proc_limits.open_files = soft_value,
                    ProcessLimitsType::LockedMemery => proc_limits.locked_memory = soft_value,
                    ProcessLimitsType::AddressSpace => proc_limits.address_space = soft_value,
                    ProcessLimitsType::FileLocks => proc_limits.file_locks = soft_value,
                    ProcessLimitsType::PendingSignals => proc_limits.pending_signals = soft_value,
                    ProcessLimitsType::MsqqueueSize => proc_limits.msgqueue_size = soft_value,
                    ProcessLimitsType::NicePriority => proc_limits.nice_priority = soft_value,
                    ProcessLimitsType::RealtimePriority => {
                        proc_limits.realtime_priority = soft_value
                    }
                    ProcessLimitsType::RealtimeTimeout => proc_limits.realtime_timeout = soft_value,
                    ProcessLimitsType::Unknown => {}
                }
            }
        }

        Ok(proc_limits)
    }
}

#[cfg(test)]
mod tests {
    use super::MAX_LIMIT_SIZE;
    use crate::process::*;
    use std::path::Path;

    #[test]
    fn proc_limits() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_limits = sys_proc.limits().expect("running proc 26231 limits");

        assert_eq!(sys_proc_limits.cpu_time, MAX_LIMIT_SIZE);
        assert_eq!(sys_proc_limits.file_size, MAX_LIMIT_SIZE);
        assert_eq!(sys_proc_limits.data_size, MAX_LIMIT_SIZE);
        assert_eq!(sys_proc_limits.stack_size, 8388608);
        assert_eq!(sys_proc_limits.core_file_size, 0);
        assert_eq!(sys_proc_limits.resident_set, MAX_LIMIT_SIZE);
        assert_eq!(sys_proc_limits.processes, 62898);
        assert_eq!(sys_proc_limits.open_files, 2048);
        assert_eq!(sys_proc_limits.locked_memory, 18446744073708503040);
        assert_eq!(sys_proc_limits.address_space, 8589934592);
        assert_eq!(sys_proc_limits.file_locks, MAX_LIMIT_SIZE);
        assert_eq!(sys_proc_limits.pending_signals, 62898);
        assert_eq!(sys_proc_limits.msgqueue_size, 819200);
        assert_eq!(sys_proc_limits.nice_priority, 0);
        assert_eq!(sys_proc_limits.realtime_priority, 0);
        assert_eq!(sys_proc_limits.realtime_timeout, MAX_LIMIT_SIZE);

        let sys_proc = collect_from(proc_path, 26234).expect("running proc 26234");
        let sys_proc_limits = sys_proc.limits();
        assert_eq!(sys_proc_limits.is_err(), true);
    }
}
