pub struct Student {
    name: String,
    schedule: Schedule,
    lesson_duration: f64,
}

impl Student {
    pub fn new(name: String, schedule: Schedule, lesson_duration: f64) -> Self {
        Student {
            name: name,
            schedule: schedule,
            lesson_duration: lesson_duration,
        }
    }
}
