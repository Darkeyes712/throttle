use std::thread;
use std::time::{Duration, Instant};

pub struct Throttle {
    interval: Duration,
    last_execution: Instant,
}

impl Throttle {
    pub fn new(interval: Duration) -> Throttle {
        Throttle {
            interval,
            last_execution: Instant::now(),
        }
    }

    pub fn execute(&mut self, rate_limit: u32) -> bool {
        let elapsed = self.last_execution.elapsed();
        if elapsed < self.interval {
            let sleep_time = self.interval - elapsed;
            thread::sleep(sleep_time);
        }

        let now = Instant::now();
        let time_diff = now.duration_since(self.last_execution);

        if time_diff.as_secs() * rate_limit as u64 >= 1 {
            self.last_execution = now;
            true
        } else {
            false
        }
    }
}
