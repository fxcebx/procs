use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct ReadBytes {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, u64>,
    max_width: usize,
}

impl ReadBytes {
    pub fn new() -> Self {
        let header = String::from("Read");
        let unit = String::from("[B/s]");
        ReadBytes {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            max_width: cmp::max(header.len(), unit.len()),
            header,
            unit,
        }
    }
}

impl Column for ReadBytes {
    fn add(&mut self, proc: &ProcessInfo) {
        let (fmt_content, raw_content) = if let (Some(proc_curr), Some(proc_prev)) =
            (&proc.procfs_io_curr, &proc.procfs_io_prev)
        {
            let interval_ms = proc.interval.as_secs() + u64::from(proc.interval.subsec_millis());
            let io = (proc_curr.read_bytes - proc_prev.read_bytes) * 1000 / interval_ms;
            let (size, unit) = unbytify::bytify(io);
            (
                format!("{}{}", size, unit.replace("i", "").replace("B", "")),
                io,
            )
        } else {
            (String::from(""), 0)
        };

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(u64);
}
