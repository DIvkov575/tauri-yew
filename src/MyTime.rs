// use crate::MyTime::Add;
// use core::ops::Add;
use chrono::*;
use std::ops::{Add, Sub};

#[derive(Default, Clone, Copy)]
pub struct MyTime {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub nano_seconds: u32,
}

impl MyTime {
    pub fn new(hours: u32, minutes: u32, seconds: u32, nano_seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            nano_seconds,
        }
    }
    pub fn naive_conv<T: chrono::Timelike>(naive_time: T) -> Self {
        Self {
            hours: naive_time.hour(),
            minutes: naive_time.minute(),
            seconds: naive_time.second(),
            nano_seconds: naive_time.nanosecond(),
        }
    }
}

impl std::fmt::Display for MyTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.hours, self.minutes, self.seconds, self.nano_seconds
        )
    }
}
impl core::ops::Sub for MyTime {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut carry = false;
        let mut hrs = 0;
        let mut mins = 0;
        let mut secs = 0;
        let mut nsecs: u64 = 0;

        // nsecs = self.nano_seconds as u64 - other.nano_seconds as u64;

        if other.nano_seconds > self.nano_seconds {
            log::info!("first");
            nsecs = 1000000000 + self.nano_seconds as u64 - other.nano_seconds as u64;
            carry = true;
        } else {
            nsecs = self.nano_seconds as u64 - other.nano_seconds as u64;
        }

        if carry && other.seconds + 1 > self.seconds {
            secs = 60 + self.seconds - other.seconds - 1;
            carry = true;
        } else if !carry && other.seconds > self.seconds {
            secs = 60 + self.seconds - other.seconds;
            carry = true;
        } else if carry && self.seconds > other.seconds + 1 {
            secs = self.seconds - other.seconds - 1;
            carry = false;
        } else if !carry && self.seconds > other.seconds {
            secs = self.seconds - other.seconds;
            carry = false;
        }

        if carry && other.minutes + 1 > self.minutes {
            mins = 60 + self.minutes - other.minutes - 1;
            carry = true;
        } else if !carry && other.minutes > self.minutes {
            mins = 60 + self.minutes - other.minutes;
            carry = true;
        } else if carry && self.minutes > other.minutes + 1 {
            mins = self.minutes - other.minutes - 1;
            carry = false;
        } else if !carry && self.minutes > other.minutes {
            mins = self.minutes - other.minutes;
            carry = false;
        }

        if carry && other.hours > self.hours + 1 {
            panic!("new day? hour mismatch");
        } else if !carry && other.hours > self.hours {
            panic!("new day? hour mismatch");
        } else if carry && self.hours > other.hours + 1 {
            hrs = self.hours - other.hours - 1;
        } else if !carry && self.hours > other.hours {
            hrs = self.hours - other.hours;
        }

        Self {
            hours: hrs,
            minutes: mins,
            seconds: secs,
            nano_seconds: nsecs as u32,
        }
    }
}

impl core::ops::Add for MyTime {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            hours: self.hours + other.hours,
            minutes: self.minutes + other.minutes,
            seconds: self.seconds + other.seconds,
            nano_seconds: self.nano_seconds + other.nano_seconds,
        }
    }
}
// }
