#[cfg(target_os = "macos")]
use libproc::libproc::proc_pid::{self, ProcType, TaskAllInfo};
#[cfg(target_os = "linux")]
use procfs;
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::thread;
#[cfg(target_os = "macos")]
use std::time::Duration;
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};
#[cfg(target_os = "linux")]
use sysinfo::{self, System, SystemExt};
#[cfg(target_os = "macos")]
use sysinfo::{Process, ProcessExt, System, SystemExt};

#[cfg(target_os = "linux")]
pub struct ProcessInfo {
    pub pid: i32,
    pub sysinfo_proc: sysinfo::Process,
    pub procfs_proc_curr: Option<procfs::Process>,
    pub procfs_proc_prev: Option<procfs::Process>,
    pub procfs_io_curr: Option<procfs::Io>,
    pub procfs_io_prev: Option<procfs::Io>,
    pub procfs_status_curr: Option<procfs::Status>,
    pub interval: Duration,
}

#[cfg(target_os = "linux")]
pub fn collect_proc(interval: Duration) -> Vec<ProcessInfo> {
    let mut pids = Vec::new();
    let mut sysinfo_procs = HashMap::new();
    let mut procfs_proc_prevs = HashMap::new();
    let mut procfs_io_prevs = HashMap::new();
    let mut time_prevs = HashMap::new();

    let system = System::new();
    for (pid, sysinfo_proc) in system.get_process_list() {
        let (procfs_proc_prev, procfs_io_prev) = if let Ok(proc) = procfs::Process::new(*pid) {
            let io = proc.io().ok();
            (Some(proc), io)
        } else {
            (None, None)
        };
        let time = Instant::now();
        pids.push(*pid);
        sysinfo_procs.insert(*pid, sysinfo_proc.clone());
        procfs_proc_prevs.insert(*pid, procfs_proc_prev);
        procfs_io_prevs.insert(*pid, procfs_io_prev);
        time_prevs.insert(*pid, time);
    }

    thread::sleep(interval);

    let mut ret = Vec::new();
    for pid in pids {
        let sysinfo_proc = sysinfo_procs.remove(&pid).unwrap();
        let procfs_proc_prev = procfs_proc_prevs.remove(&pid).unwrap();
        let procfs_io_prev = procfs_io_prevs.remove(&pid).unwrap();
        let time_prev = time_prevs.remove(&pid).unwrap();

        let (procfs_proc_curr, procfs_io_curr, procfs_status_curr) =
            if let Ok(proc) = procfs::Process::new(pid) {
                let io = proc.io().ok();
                let status = proc.status().ok();
                (Some(proc), io, status)
            } else {
                (None, None, None)
            };

        let time_curr = Instant::now();
        let interval = time_curr - time_prev;

        let proc = ProcessInfo {
            pid,
            sysinfo_proc,
            procfs_proc_curr,
            procfs_proc_prev,
            procfs_io_curr,
            procfs_io_prev,
            procfs_status_curr,
            interval,
        };

        ret.push(proc);
    }

    ret
}

#[cfg(target_os = "macos")]
pub struct ProcessInfo {
    pub curr_proc: TaskAllInfo,
    pub proc: Process,
}

#[cfg(target_os = "macos")]
pub fn collect_proc(_interval: Duration) -> Vec<ProcessInfo> {
    let mut ret = Vec::new();
    let mut system = System::new();

    for (pid, proc) in system.get_process_list() {
        if let Ok(curr_proc) = proc_pid::pidinfo::<TaskAllInfo>(*pid as i32, 0) {
            let proc = ProcessInfo {
                curr_proc,
                proc: proc.clone(),
            };
            ret.push(proc);
        }
    }

    ret
}
