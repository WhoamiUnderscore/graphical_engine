use std::time::{Instant, Duration};
use std::thread;

pub struct Time {
    pub start: Instant,
    pub now: Instant,
    pub time_now: f64,
    pub delta_time: f64,
    pub last_time: f64
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Time {
            start: now,
            now: now,
            time_now: 0.0,
            delta_time: 0.0,
            last_time: 0.016
        }
    }

    pub fn update(&mut self) {
        self.now = Instant::now();
        self.time_now = (self.now - self.start).as_secs_f64();
        self.delta_time = self.time_now - self.last_time;
        self.last_time = self.time_now;
        
        thread::sleep(Duration::from_millis(16));
    }
}
