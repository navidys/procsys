use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// Proc represent a single process
#[derive(Debug, Serialize)]
pub struct Proc {
    id: usize,
    path: PathBuf,
}

impl Proc {
    fn new(id: usize, path: PathBuf) -> Self {
        Self { id, path }
    }

    pub fn pid(&self) -> usize {
        self.id
    }

    /// returns the command name of a process
    pub fn comm(&self) -> CollectResult<String> {
        match utils::collect_info_string("comm", Path::new(&self.path)) {
            Ok(c) => Ok(c.unwrap_or_default().trim().to_string()),
            Err(err) => Err(err),
        }
    }
}

/// Collects all available processes running on system.
/// # Example
/// ```
/// use procsys::proc;
///
/// let procs = proc::collect_all().expect("system processes");
/// let json_output = serde_json::to_string_pretty(&procs).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect_all() -> CollectResult<Vec<Proc>> {
    let proc_path = Path::new("/proc");
    collect_from(proc_path)
}

/// collect and return a specific process
/// # Example
/// ```
/// use procsys::proc;
///
/// let proc = proc::collect(1).expect("process pid 1");
/// println!("pid: {}", proc.pid());
/// println!("\t comm: {}", proc.comm().unwrap_or_default());
///
/// ```
pub fn collect(pid: usize) -> CollectResult<Proc> {
    let proc_path = format!("/proc/{}/", pid);
    let proc_dir_path = PathBuf::from(proc_path);

    if proc_dir_path.as_path().is_dir() {
        return Ok(Proc::new(pid, proc_dir_path));
    }

    Err(MetricError::ProcessNotFound(pid))
}

fn collect_from(base_path: &Path) -> CollectResult<Vec<Proc>> {
    let mut sysprocs = Vec::new();

    for file_info in utils::list_dir_content(base_path, "", "proc") {
        if let Ok(pid) = file_info.parse::<usize>() {
            let proc_path = format!("/proc/{}/", pid);
            let proc_dir_path = PathBuf::from(proc_path);

            if proc_dir_path.as_path().is_dir() {
                sysprocs.push(Proc::new(pid, proc_dir_path));
            }
        }
    }

    Ok(sysprocs)
}
