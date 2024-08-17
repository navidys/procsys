use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    proc::Proc,
    utils,
};

enum ProcIOType {
    RChar,
    WChar,
    SyscR,
    SyscW,
    ReadBytes,
    WriteBytes,
    CancelledWriteBytes,
    Unknown,
}

impl ProcIOType {
    fn from(name: &str) -> ProcIOType {
        match name {
            "rchar" => ProcIOType::RChar,
            "wchar" => ProcIOType::WChar,
            "syscr" => ProcIOType::SyscR,
            "syscw" => ProcIOType::SyscW,
            "read_bytes" => ProcIOType::ReadBytes,
            "write_bytes" => ProcIOType::WriteBytes,
            "cancelled_write_bytes" => ProcIOType::CancelledWriteBytes,
            _ => ProcIOType::Unknown,
        }
    }
}

/// ProcIO models the content of /proc/\<pid\>/io
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcIO {
    pub rchar: u64,
    pub wchar: u64,
    pub syscr: u64,
    pub syscw: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub cancelled_write_bytes: i64,
}

impl ProcIO {
    fn new() -> Self {
        Default::default()
    }
}

impl Proc {
    /// returns proc IO stats
    pub fn io(&self) -> CollectResult<ProcIO> {
        let mut proc_io = ProcIO::new();
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
            match ProcIOType::from(item_fields[0]) {
                ProcIOType::RChar => proc_io.rchar = item_value.parse::<u64>().unwrap_or_default(),
                ProcIOType::WChar => proc_io.wchar = item_value.parse::<u64>().unwrap_or_default(),
                ProcIOType::SyscR => proc_io.syscr = item_value.parse::<u64>().unwrap_or_default(),
                ProcIOType::SyscW => proc_io.syscw = item_value.parse::<u64>().unwrap_or_default(),
                ProcIOType::ReadBytes => {
                    proc_io.read_bytes = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcIOType::WriteBytes => {
                    proc_io.write_bytes = item_value.parse::<u64>().unwrap_or_default()
                }
                ProcIOType::CancelledWriteBytes => {
                    proc_io.cancelled_write_bytes = item_value.parse::<i64>().unwrap_or_default()
                }
                ProcIOType::Unknown => {}
            }
        }

        Ok(proc_io)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::proc::*;

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
