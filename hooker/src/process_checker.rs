use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub fn get_ts3_server_pid() -> Option<u32> {
    let s = System::new_all();
    if let Some(process) = s.processes_by_name("ts3server.exe").next() {
        return Some(process.pid().as_u32());
    }
    None
}
