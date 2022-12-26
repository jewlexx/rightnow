use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RightNow {
    pub since_epoch: Duration,
}

impl RightNow {
    #[inline]
    pub fn now() -> Self {
        RightNow {
            since_epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards"),
        }
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.since_epoch.as_secs()
    }

    #[inline]
    pub fn as_secs_f64(&self) -> f64 {
        self.since_epoch.as_secs_f64()
    }

    #[inline]
    pub fn as_secs_f32(&self) -> f32 {
        self.since_epoch.as_secs_f32()
    }

    #[inline]
    pub fn as_millis(&self) -> u128 {
        self.since_epoch.as_millis()
    }

    #[inline]
    pub fn as_micros(&self) -> u128 {
        self.since_epoch.as_micros()
    }

    #[inline]
    pub fn as_nanos(&self) -> u128 {
        self.since_epoch.as_nanos()
    }
}
