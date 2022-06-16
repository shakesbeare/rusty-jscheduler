mod time_slot;
use std::vec::Vec;
use crate::time_slot::TimeSlot;
use chrono::NaiveDate;


pub struct Schedule {
    slots: Vec<TimeSlot>,
}

impl Schedule {
    pub fn new() -> Self {
        let slots = Vec<TimeSlot>::new();
        Schedule { slots: slots }
    }

    pub fn add_slot(&self, slot: &TimeSlot) -> TimeSlot {
        // Add the new timeslot
        self.slots.push(slot);
        // Sort the vector
        self.slots.sort_unstable();
    }

    pub fn check_availability(&self, other: &Schedule, duration: f64) -> bool {
        for own_slot in self.slots {
            for other_slot in other.slots {
                if own_slot.available_slot_exists(other, duration) {
                    return true;
                } 
            }
        }

        false
    }
}
