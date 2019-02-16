use crate::process::ProcessInfo;
use crate::{column_default, Column};
use chrono::{offset::TimeZone, DateTime, Local};
use std::cmp;
use std::collections::HashMap;
use sysinfo::ProcessExt;

pub struct StartTime {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, DateTime<Local>>,
    max_width: usize,
}

impl StartTime {
    pub fn new() -> Self {
        let header = String::from("Start");
        let unit = String::from("");
        StartTime {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            max_width: cmp::max(header.len(), unit.len()),
            header,
            unit,
        }
    }
}

impl Column for StartTime {
    fn add(&mut self, proc: &ProcessInfo) {
        let start_time = Local.timestamp(proc.sysinfo_proc.start_time() as i64, 0);
        let raw_content = start_time;
        let fmt_content = format!("{}", start_time.format("%Y/%m/%d %H:%M"));

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(DateTime<Local>);
}
