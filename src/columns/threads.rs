use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct Threads {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, i64>,
    max_width: usize,
}

impl Threads {
    pub fn new() -> Self {
        let header = String::from("Threads");
        let unit = String::from("");
        Threads {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            max_width: cmp::max(header.len(), unit.len()),
            header,
            unit,
        }
    }
}

impl Column for Threads {
    fn add(&mut self, proc: &ProcessInfo) {
        let raw_content = if let Some(proc) = &proc.procfs_proc_curr {
            proc.stat.num_threads
        } else {
            0
        };
        let fmt_content = format!("{}", raw_content);

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(i64);
}
