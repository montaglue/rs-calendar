use std::time::{SystemTime, UNIX_EPOCH};

const DAY: usize = 60 * 60 * 24;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Time {
    seconds: usize,
}

impl Time {
    pub fn now() -> Time {
        Time {
            seconds: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
        }
    }

    pub fn day(self) -> Day {
        Day {
            time: self.day_begin(),
        }
    }

    pub fn day_begin(self) -> Time {
        Time {
            seconds: self.seconds - self.seconds % DAY,
        }
    }

    pub fn next_day(self) -> Time {
        Time {
            seconds: self.day_begin().seconds + DAY
        }
    }

    pub fn add(self, duration: Time) -> Time {
        Time {
            seconds: self.seconds + duration.seconds,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Day {
    time: Time,
}

impl Day {
    pub fn time(&self) -> &Time {
        &self.time
    }
}