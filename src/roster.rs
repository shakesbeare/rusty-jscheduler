use std::cmp;
#[allow(dead_code)]
use std::vec::Vec;

pub struct Roster {
    teacher_schedule: Schedule,
    students: Vec<Student>,
}

impl Roster {
    pub fn new(teacher_schedule: Schedule, student_schedules: Vec<Schedule>) {
        Roster {
            teacher_schedule: teacher_schedule,
            student_schedules: student_schedules,
        }
    }

    // Creates a schedule given student availability and teacher availability
    pub fn create_schedule(&self, increment: f64) -> Vec<Schedule> {
        for student in students {
            if !student.schedule.check_availability(self.teacher_schedule) {
                continue;
            }
        }
    }
}
