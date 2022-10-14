use std::time::SystemTime;

pub struct Timer {
    pub start_time: SystemTime
}

impl Timer {
    pub fn new() -> Self {
        let sys_time = std::time::SystemTime::now();
        return Timer { start_time: sys_time }
    }
}