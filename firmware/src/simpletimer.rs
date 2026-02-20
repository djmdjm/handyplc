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

#[cfg(test)]
mod tests {
    use super::*;
    use fugit::ExtU32;

    #[test]
    fn test_timer_not_expired_immediately() {
        let now = 1000;
        let duration = 100.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(!timer.expired(now));
    }

    #[test]
    fn test_timer_expired_after_duration() {
        let now = 1000;
        let duration = 100.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(timer.expired(now + 100));
    }

    #[test]
    fn test_timer_expired_long_after_duration() {
        let now = 1000;
        let duration = 100.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(timer.expired(now + 200));
    }

    #[test]
    fn test_timer_not_expired_before_duration() {
        let now = 1000;
        let duration = 100.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(!timer.expired(now + 99));
    }

    #[test]
    fn test_zero_duration() {
        let now = 1000;
        let duration = 0.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(timer.expired(now));
    }

    #[test]
    fn test_timer_boundary_at_now() {
        let now = 1000;
        let duration = 1.millis();
        let timer = SimpleTimer::start(now, duration);
        assert!(!timer.expired(now));
        assert!(timer.expired(now + 1));
    }
}


