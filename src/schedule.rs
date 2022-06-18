#![allow(dead_code)]

use crate::time_slot::TimeSlot;
use std::vec::Vec;

pub struct Schedule {
    slots: Vec<TimeSlot>,
    slot_titles: Vec<String>,
}

impl Schedule {
    pub fn new() -> Self {
        let slots = Vec::<TimeSlot>::new();
        let titles = Vec::<String>::new();
        Schedule {
            slots: slots,
            slot_titles: titles,
        }
    }

    pub fn add_slot(&mut self, slot: &TimeSlot) {
        // Add the new timeslot
        self.slots.push(slot.clone());
        // Sort the vector
        self.slots.sort_unstable();
    }

    // Returns true if *any* matching time slot exists
    pub fn check_availability(&self, other: &Schedule, duration: f64) -> bool {
        for own_slot in &self.slots {
            for other_slot in &other.slots {
                if own_slot.available_slot_exists(&other_slot, duration) {
                    return true;
                }
            }
        }
        false
    }
}
