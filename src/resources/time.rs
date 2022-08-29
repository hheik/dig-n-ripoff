use std::time::{Duration, SystemTime};

pub struct Time {
    pub delta_time: Duration,
    pub phys_delta_time: Duration,
    pub lifetime: SystemTime,
    pub frame: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta_time: Duration::new(0, 0),
            phys_delta_time: Duration::new(0, 1_000_000_000u32 / 60),
            lifetime: std::time::SystemTime::now(),
            frame: 0,
        }
    }
}
