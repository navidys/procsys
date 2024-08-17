use std::{
    fs::read_link,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// Proc represent a single process
#[derive(Debug, Serialize)]
pub struct Process {
    id: usize,
    path: PathBuf,
}

impl Process {
    fn new(id: usize, path: PathBuf) -> Self {
        Self { id, path }
    }

    pub fn pid(&self) -> usize {
        self.id
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    /// returns the command name of a process
    pub fn comm(&self) -> CollectResult<String> {
        match utils::collect_info_string("comm", &self.path) {
            Ok(c) => Ok(c.unwrap_or_default().trim().to_string()),
            Err(err) => Err(err),
        }
    }

    /// returns the wchan (wait channel) of a process
    pub fn wchan(&self) -> CollectResult<String> {
        match utils::collect_info_string("wchan", &self.path) {
            Ok(c) => {
                let mut wchan = String::new();
                let wchan_data = c.unwrap_or_default().trim().to_string();

                if wchan_data != "0" {
                    wchan = wchan_data;
                }

                Ok(wchan)
            }
            Err(err) => Err(err),
        }
    }

    /// returns the command line of a process
    pub fn cmdline(&self) -> CollectResult<Vec<String>> {
        match utils::collect_info_string("cmdline", &self.path) {
            Ok(c) => {
                let proc_cmdline = c
                    .unwrap_or_default()
                    .trim_end_matches("\x00")
                    .split("\x00")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();

                Ok(proc_cmdline)
            }
            Err(err) => Err(err),
        }
    }

    /// returns the process environments from `/proc/<pid>/environ`
    pub fn environ(&self) -> CollectResult<Vec<String>> {
        match utils::collect_info_string("environ", &self.path) {
            Ok(c) => {
                let proc_cmdline = c
                    .unwrap_or_default()
                    .trim_end_matches("\x00")
                    .split("\x00")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();

                Ok(proc_cmdline)
            }
            Err(err) => Err(err),
        }
    }

    /// returns the absolute path of the executable command of a process
    pub fn executable(&self) -> CollectResult<PathBuf> {
        let mut proc_path = self.path.clone();
        proc_path.push("exe");

        match read_link(&proc_path) {
            Ok(c) => {
                if c.exists() {
                    return Ok(c);
                }

                Err(MetricError::PathNotFound(c))
            }
            Err(err) => Err(MetricError::IOError(proc_path, err)),
        }
    }

    /// returns the absolute path to the current working directory of the process
    pub fn cwd(&self) -> CollectResult<PathBuf> {
        let mut proc_path = self.path.clone();
        proc_path.push("cwd");

        match read_link(&proc_path) {
            Ok(c) => {
                if c.exists() {
                    return Ok(c);
                }

                Err(MetricError::PathNotFound(c))
            }
            Err(err) => Err(MetricError::IOError(proc_path, err)),
        }
    }

    /// returns the absolute path to the process's root directory (as set by chroot)
    pub fn root_dir(&self) -> CollectResult<PathBuf> {
        let mut proc_path = self.path.clone();
        proc_path.push("root");

        match read_link(&proc_path) {
            Ok(c) => {
                if c.exists() {
                    return Ok(c);
                }

                Err(MetricError::PathNotFound(c))
            }
            Err(err) => Err(MetricError::IOError(proc_path, err)),
        }
    }
}

/// Collects all available processes running on system.
/// # Example
/// ```
/// use procsys::process;
///
/// let procs = process::collect_all().expect("system processes");
/// for proc in procs {
///     println!("pid: {}", proc.pid());
///     println!("\t comm: {}", proc.comm().unwrap_or_default());
///     println!("\t wchan: {}", proc.wchan().unwrap_or_default());
///     println!("\t executable: {:?}", proc.executable().unwrap_or_default());
///     println!("\t cwd: {:?}", proc.cwd().unwrap_or_default());
///     println!("\t root: {:?}", proc.root_dir().unwrap_or_default());
/// }
///
/// ```
pub fn collect_all() -> CollectResult<Vec<Process>> {
    let proc_path = Path::new("/proc");
    collect_all_from(proc_path)
}

/// collect and return a specific process
/// # Example
/// ```
/// use procsys::process;
///
/// let proc = process::collect(1).expect("process pid 1");
/// println!("pid: {}", proc.pid());
/// println!("\t comm: {}", proc.comm().unwrap_or_default());
///
/// ```
pub fn collect(pid: usize) -> CollectResult<Process> {
    let proc_path = Path::new("/proc");
    collect_from(proc_path, pid)
}

fn collect_all_from(base_path: &Path) -> CollectResult<Vec<Process>> {
    let mut sysprocs = Vec::new();

    for file_info in utils::list_dir_content(base_path, "", "proc") {
        if let Ok(pid) = file_info.parse::<usize>() {
            let mut proc_dir_path = PathBuf::from(base_path);
            proc_dir_path.push(format!("{}", pid));

            if proc_dir_path.as_path().is_dir() {
                sysprocs.push(Process::new(pid, proc_dir_path));
            }
        }
    }

    Ok(sysprocs)
}

pub fn collect_from(base_path: &Path, pid: usize) -> CollectResult<Process> {
    let mut proc_dir_path = PathBuf::from(base_path);
    proc_dir_path.push(format!("{}", pid));

    if proc_dir_path.as_path().is_dir() {
        return Ok(Process::new(pid, proc_dir_path));
    }

    Err(MetricError::ProcessNotFound(pid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_collect_all() {
        let sysprocs =
            collect_all_from(Path::new("test_data/fixtures/proc")).expect("running procs");
        assert_eq!(sysprocs.len(), 7);
    }

    #[test]
    fn proc_collect() {
        let proc_path = Path::new("test_data/fixtures/proc");

        let sys_single_proc = collect_from(proc_path, 2);
        assert_eq!(sys_single_proc.is_err(), true);

        let sys_single_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        assert_eq!(sys_single_proc.cwd().unwrap(), PathBuf::from("/usr/bin/"));
        assert_eq!(sys_single_proc.comm().unwrap(), "vim");
        assert_eq!(sys_single_proc.wchan().unwrap(), "poll_schedule_timeout");
        assert_eq!(
            sys_single_proc.executable().unwrap(),
            PathBuf::from("/usr/bin/vim"),
        );
        assert_eq!(sys_single_proc.root_dir().unwrap(), PathBuf::from("/"));
        assert_eq!(
            sys_single_proc.cmdline().unwrap(),
            ["vim", "test.go", "+10"],
        );
        assert_eq!(
            sys_single_proc.environ().unwrap(),
            ["PATH=/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin", "HOSTNAME=cd24e11f73a5", "TERM=xterm", "GOLANG_VERSION=1.12.5", "GOPATH=/go", "HOME=/root"],
        );

        let sys_single_proc = collect_from(proc_path, 26232).expect("running proc 26232");
        assert_eq!(sys_single_proc.cwd().is_err(), true);
        assert_eq!(sys_single_proc.root_dir().is_err(), true);
    }
}
