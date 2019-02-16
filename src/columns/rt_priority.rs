use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct RtPriority {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, u32>,
    max_width: usize,
}

impl RtPriority {
    pub fn new() -> Self {
        let header = String::from("RT Priority");
        let unit = String::from("");
        RtPriority {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            max_width: cmp::max(header.len(), unit.len()),
            header,
            unit,
        }
    }
}

impl Column for RtPriority {
    fn add(&mut self, proc: &ProcessInfo) {
        let (raw_content, fmt_content) = if let Some(proc) = &proc.procfs_proc_curr {
            if let Some(p) = proc.stat.rt_priority {
                (p, format!("{}", p))
            } else {
                (Default::default(), String::from(""))
            }
        } else {
            (Default::default(), String::from(""))
        };

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(u32);
}
