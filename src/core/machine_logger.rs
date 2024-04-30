pub struct MachineLogEntry {
    logs: [Option<String>; 8],
    logcnt: u32,
}

pub struct MachineLogger {
    pub logs: [MachineLogEntry; 1024],
    pub start: u32,
    pub end: u32,
}

impl MachineLogEntry {
    pub fn new() -> MachineLogEntry {
        const DEFAULT_LOG: Option<String> = None;
        MachineLogEntry {
            logs: [DEFAULT_LOG; 8],
            logcnt: 0,
        }
    }
}
