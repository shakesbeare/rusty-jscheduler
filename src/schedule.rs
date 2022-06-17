#![allow(dead_code)]

use crate::time_slot::TimeSlot;
use std::vec::Vec;

pub struct Schedule {
    slots: Vec<TimeSlot>,
}

impl Schedule {
    pub fn new() -> Self {
        let slots = Vec::<TimeSlot>::new();
        Schedule { slots: slots }
    }

    pub fn add_slot(&mut self, slot: &TimeSlot) {
        // Add the new timeslot
        self.slots.push(slot.clone());
        // Sort the vector
        self.slots.sort_unstable();
    }

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
