use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

/// ProcessCgroup models one line from /proc/\[pid\]/cgroup
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessCgroup {
    pub hierarchy_id: usize,
    pub controllers: Vec<String>,
    pub path: String,
}

impl ProcessCgroup {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// cgroup reads from /proc/\<pid\>/cgroup and returns cgroup information of the process
    pub fn cgroup(&self) -> CollectResult<Vec<ProcessCgroup>> {
        let mut proc_cgroups = Vec::new();
        let proc_cgroup_path_str = format!("{:?}", self.path());
        let proc_cgroup_file = format!("{}/cgroup", proc_cgroup_path_str.replace("\"", ""));

        for line in utils::read_file_lines(&proc_cgroup_file)? {
            let item_fields: Vec<&str> = line.trim().split(':').collect();
            if item_fields.len() != 3 {
                return Err(MetricError::InvalidFieldNumberError(
                    "process cgroup".to_string(),
                    item_fields.len(),
                    line,
                ));
            }

            let mut proc_cgroup = ProcessCgroup::new();
            proc_cgroup.path = item_fields[2].trim().to_string();

            match item_fields[0].parse::<usize>() {
                Ok(v) => proc_cgroup.hierarchy_id = v,
                Err(e) => return Err(MetricError::ParseIntError(item_fields[0].to_string(), e)),
            }

            if item_fields[1].trim() != "" {
                proc_cgroup.controllers = item_fields[1]
                    .trim()
                    .split(",")
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
            }

            proc_cgroups.push(proc_cgroup);
        }

        Ok(proc_cgroups)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::process::*;

    #[test]
    fn proc_cgroup() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_cgroup = sys_proc.cgroup().expect("running proc 26231 cgroup stat");

        assert_eq!(sys_proc_cgroup.len(), 1);
        assert_eq!(sys_proc_cgroup[0].hierarchy_id, 1);
        assert_eq!(sys_proc_cgroup[0].controllers.len(), 0);
        assert_eq!(sys_proc_cgroup[0].path, "/user.slice/user-1000.slice/user@1000.service/app.slice/app-org.gnome.Terminal.slice/vte-spawn-fd5b6c83-c316-470a-9732-4db75febce50.scope");

        let sys_proc = collect_from(proc_path, 26232).expect("running proc 26232");
        let sys_proc_cgroup = sys_proc.cgroup();
        assert_eq!(sys_proc_cgroup.is_err(), true);
    }
}
