use core::fmt;
use std::error::Error;
use chrono::NaiveDateTime;
use std::cmp;
use std::cmp::Ordering;

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
        self.start_time.timestamp().cmp(&other.start_time.timestamp())
    }
}

impl PartialOrd for TimeSlot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start_time.timestamp().cmp(&other.start_time.timestamp()))
    }
}

impl TimeSlot {
    pub fn new(start_time: NaiveDateTime, duration: f64, available: bool) -> Self {
        TimeSlot { available: available, start_time: start_time, end_time: NaiveDateTime::from_timestamp(start_time.timestamp() + ((duration * 3600.0) as i64), 0) }
    }
    // Checks if the given TimeSlot is available at the time of the self time slot
    // requested_time_slot.check_available(other_time_slot);
    // Note: the `other` time slot shouldb be larger than the `self` time slot
    pub fn check_available(&self, other: &TimeSlot) -> Result<bool, TimeSlotError> {
        //println!("{}-{}, {}-{}", self.start_time.timestamp(), self.end_time.timestamp(), other.start_time.timestamp(), other.end_time.timestamp());

        if !self.available || !other.available {
            return Ok(false);
        }

        if self.start_time.timestamp() >= other.start_time.timestamp() && self.end_time.timestamp() <= other.end_time.timestamp() {
            Ok(true) 
        } else {
            Ok(false)
        }
    }
    
    // This functions similarly to `check_available` except that both `self` and `other` can be of
    // any size
    pub fn available_slot_exists(&self, other: &TimeSlot, duration: f64) -> Result<bool, TimeSlotError>{
        if !self.available || !other.available {
            return Ok(false);
        }

        let left_endpoint = cmp::max(self.start_time.timestamp(), other.start_time.timestamp());
        let right_endpoint = cmp::min(self.end_time.timestamp(), other.end_time.timestamp());
        let total_available_duration = right_endpoint - left_endpoint;
        
        if total_available_duration >= (duration * 3600.0) as i64 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Alias for TimeSlot.available_slot_exists()
pub fn available_slot_exists(first: &TimeSlot, second: &TimeSlot, duration: f64) -> Result<bool, TimeSlotError>{
    first.available_slot_exists(second, duration)
}
