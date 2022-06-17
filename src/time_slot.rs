#![allow(dead_code)]

use chrono::NaiveDateTime;
use core::fmt;
use std::cmp;
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug)]
pub struct TimeSlotError {
    message: std::string::String,
}

impl std::fmt::Display for TimeSlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TimeSlotError {}

#[derive(Eq, PartialEq)]
pub struct TimeSlot {
    available: bool,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

impl Ord for TimeSlot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time
            .timestamp()
            .cmp(&other.start_time.timestamp())
    }
}

impl PartialOrd for TimeSlot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.start_time
                .timestamp()
                .cmp(&other.start_time.timestamp()),
        )
    }
}

impl Clone for TimeSlot {
    fn clone(&self) -> Self {
        TimeSlot::from_date_time(self.start_time, self.end_time, self.available)
    }
}

impl TimeSlot {
    pub fn new(start_time: NaiveDateTime, duration: f64, available: bool) -> Self {
        TimeSlot {
            available: available,
            start_time: start_time,
            end_time: NaiveDateTime::from_timestamp(
                start_time.timestamp() + ((duration * 3600.0) as i64),
                0,
            ),
        }
    }
    pub fn from_date_time(
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        available: bool,
    ) -> Self {
        TimeSlot {
            available: available,
            start_time: start_time,
            end_time: end_time,
        }
    }
    // Checks if the given TimeSlot is available at the time of the self time slot
    // requested_time_slot.check_available(other_time_slot);
    // Note: the `other` time slot shouldb be larger than the `self` time slot
    pub fn check_available(&self, other: &TimeSlot) -> bool {
        //println!("{}-{}, {}-{}", self.start_time.timestamp(), self.end_time.timestamp(), other.start_time.timestamp(), other.end_time.timestamp());

        if !self.available || !other.available {
            return false;
        }

        if self.start_time.timestamp() >= other.start_time.timestamp()
            && self.end_time.timestamp() <= other.end_time.timestamp()
        {
            true
        } else {
            false
        }
    }

    pub fn from_timestamp(start_time: i64, duration: f64, available: bool) -> Self {
        TimeSlot {
            available: available,
            start_time: NaiveDateTime::from_timestamp(start_time, 0),
            end_time: NaiveDateTime::from_timestamp(start_time + (duration * 3600.0) as i64, 0),
        }
    }
    // This functions similarly to `check_available` except that both `self` and `other` can be of
    // any size
    pub fn available_slot_exists(&self, other: &TimeSlot, duration: f64) -> bool {
        if !self.available || !other.available {
            return false;
        }

        let left_endpoint = cmp::max(self.start_time.timestamp(), other.start_time.timestamp());
        let right_endpoint = cmp::min(self.end_time.timestamp(), other.end_time.timestamp());
        let total_available_duration = right_endpoint - left_endpoint;

        if total_available_duration >= (duration * 3600.0) as i64 {
            true
        } else {
            false
        }
    }
}

// Alias for TimeSlot.available_slot_exists()
pub fn available_slot_exists(first: &TimeSlot, second: &TimeSlot, duration: f64) -> bool {
    first.available_slot_exists(second, duration)
}
