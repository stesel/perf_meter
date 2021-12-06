use std::time::{Duration, Instant};

pub struct PerfMeter {
    pub interval: Duration,
    pub ops_count: usize,
    pub start: Instant,
}

impl PerfMeter {
    pub fn new(interval_sec: u64) -> Self {
        Self {
            interval: Duration::new(interval_sec, 0),
            ops_count: 0,
            start: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        self.ops_count += 1;
        let now = Instant::now();
        if now - self.start >= self.interval {
            println!("performed {} ops per {} sec", self.ops_count, self.interval.as_secs());
            self.start = now;
            self.ops_count = 0;
        }
    }
}
