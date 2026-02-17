//! Very simple timer based on a monotonic 1kHz tick.
use fugit::Duration;

pub struct SimpleTimer {
    expiry: i64,
}

impl SimpleTimer {
    pub const fn start(now: i64, duration: Duration<u32, 1, 1_000>) -> SimpleTimer {
        SimpleTimer { expiry: now + duration.ticks() as i64 }
    }
    pub fn expired(&self, now: i64) -> bool {
        self.expiry <= now
    }
}


