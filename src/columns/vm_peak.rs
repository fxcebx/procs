use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct VmPeak {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, u64>,
    max_width: usize,
}

impl VmPeak {
    pub fn new() -> Self {
        let header = String::from("VmPeak");
        let unit = String::from("[bytes]");
        VmPeak {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            max_width: cmp::max(header.len(), unit.len()),
            header,
            unit,
        }
    }
}

impl Column for VmPeak {
    fn add(&mut self, proc: &ProcessInfo) {
        let (raw_content, fmt_content) = if let Some(ref curr_status) = proc.procfs_status_curr {
            if let Some(val) = curr_status.vmpeak {
                let val = val * 1024;
                let (size, unit) = unbytify::bytify(val);
                (
                    val,
                    format!("{}{}", size, unit.replace("i", "").replace("B", "")),
                )
            } else {
                (0, String::from(""))
            }
        } else {
            (0, String::from(""))
        };

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(u64);
}
