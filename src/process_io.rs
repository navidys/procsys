use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

enum ProcessIOType {
    RChar,
    WChar,
    SyscR,
    SyscW,
    ReadBytes,
    WriteBytes,
    CancelledWriteBytes,
    Unknown,
}

impl ProcessIOType {
    fn from(name: &str) -> ProcessIOType {
        match name {
            "rchar" => ProcessIOType::RChar,
            "wchar" => ProcessIOType::WChar,
            "syscr" => ProcessIOType::SyscR,
            "syscw" => ProcessIOType::SyscW,
            "read_bytes" => ProcessIOType::ReadBytes,
            "write_bytes" => ProcessIOType::WriteBytes,
            "cancelled_write_bytes" => ProcessIOType::CancelledWriteBytes,
            _ => ProcessIOType::Unknown,
        }
    }
}

/// ProcIO models the content of /proc/\<pid\>/io
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessIO {
    pub rchar: u64,
    pub wchar: u64,
    pub syscr: u64,
    pub syscw: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub cancelled_write_bytes: i64,
}

impl ProcessIO {
    fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// returns proc IO stats
    pub fn io(&self) -> CollectResult<ProcessIO> {
        let mut proc_io = ProcessIO::new();
        let proc_io_path_str = format!("{:?}", self.path());
        let proc_io_file = format!("{}/io", proc_io_path_str.replace("\"", ""));
        for line in utils::read_file_lines(&proc_io_file)? {
            let item_fields: Vec<&str> = line.trim().split(':').filter(|s| !s.is_empty()).collect();

            if item_fields.len() != 2 {
                return Err(MetricError::InvalidFieldNumberError(
                    "proc io".to_string(),
                    item_fields.len(),
                    line,
                ));
            }

            let item_value = item_fields[1].trim();
            match ProcessIOType::from(item_fields[0]) {
                ProcessIOType::RChar => {
                    proc_io.rchar = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::WChar => {
                    proc_io.wchar = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::SyscR => {
                    proc_io.syscr = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::SyscW => {
                    proc_io.syscw = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::ReadBytes => {
                    proc_io.read_bytes = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::WriteBytes => {
                    proc_io.write_bytes = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcessIOType::CancelledWriteBytes => {
                    proc_io.cancelled_write_bytes = item_value.parse::<i64>().unwrap_or_default()
                }
                ProcessIOType::Unknown => {}
            }
        }

        Ok(proc_io)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::process::*;

    #[test]
    fn proc_io() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_io = sys_proc.io().expect("running proc 26231 io stat");

        assert_eq!(sys_proc_io.rchar, 750339);
        assert_eq!(sys_proc_io.wchar, 818609);
        assert_eq!(sys_proc_io.syscr, 7405);
        assert_eq!(sys_proc_io.syscw, 5245);
        assert_eq!(sys_proc_io.read_bytes, 1024);
        assert_eq!(sys_proc_io.write_bytes, 2048);
        assert_eq!(sys_proc_io.cancelled_write_bytes, -1024);
    }
}
