mod schedule;
mod time_slot;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::schedule::Schedule;
    use crate::time_slot::TimeSlot;
    use chrono::NaiveDate;

    #[test]
    fn time_slot_marked_busy() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(14, 0, 0);
        let first_time_slot = TimeSlot::new(start_time, 3., false);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(15, 0, 0);
        let second_time_slot = TimeSlot::new(start_time, 1., true);

        assert!(!second_time_slot.check_available(&first_time_slot));
    }

    #[test]
    fn time_slot_marked_available() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let available_time_slot = TimeSlot::new(start_time, 9., true);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(15, 0, 0);
        let scheduled_time_slot = TimeSlot::new(start_time, 1., true);

        assert!(scheduled_time_slot.check_available(&available_time_slot));
    }

    #[test]
    fn no_time_slot_means_unavailable() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(13, 0, 0);
        let available_at_wrong_time = TimeSlot::new(start_time, 2., true);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(16, 0, 0);
        let scheduled_time_slot = TimeSlot::new(start_time, 1., true);

        assert!(!scheduled_time_slot.check_available(&available_at_wrong_time));
    }

    #[test]
    fn time_slots_contain_some_available_time_slot() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(13, 0, 0);
        let teacher_slot = TimeSlot::new(start_time, 9., true);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let student_slot = TimeSlot::new(start_time, 9., true);

        assert!(student_slot.available_slot_exists(&teacher_slot, 1.));
    }

    #[test]
    fn available_slot_exists_but_student_unavailable() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(13, 0, 0);
        let teacher_slot = TimeSlot::new(start_time, 9., true);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let student_slot = TimeSlot::new(start_time, 9., false);

        assert!(!student_slot.available_slot_exists(&teacher_slot, 1.));
    }

    #[test]
    fn available_slot_exists_but_teacher_unavailable() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(13, 0, 0);
        let teacher_slot = TimeSlot::new(start_time, 9., false);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let student_slot = TimeSlot::new(start_time, 9., true);

        assert!(!student_slot.available_slot_exists(&teacher_slot, 1.));
    }

    #[test]
    fn time_slots_do_not_contain_available_slot() {
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(13, 0, 0);
        let teacher_slot = TimeSlot::new(start_time, 3., true);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(9, 0, 0);
        let student_slot = TimeSlot::new(start_time, 3., true);

        assert!(!student_slot.available_slot_exists(&teacher_slot, 1.));
    }

    #[test]
    fn two_schedules_have_availability() {
        let mut teacher_schedule = Schedule::new();
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let slot = TimeSlot::new(start_time, 3., true);
        teacher_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(11, 0, 0);
        let slot = TimeSlot::new(start_time, 1., false);
        teacher_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(12, 0, 0);
        let slot = TimeSlot::new(start_time, 5., true);
        teacher_schedule.add_slot(&slot);

        // student schedule
        let mut student_schedule = Schedule::new();
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let slot = TimeSlot::new(start_time, 1., true);
        student_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(9, 0, 0);
        let slot = TimeSlot::new(start_time, 8., false);
        student_schedule.add_slot(&slot);

        let has_availability = teacher_schedule.check_availability(&student_schedule, 1.);
        assert!(has_availability);
    }

    #[test]
    fn two_schedules_dont_have_availability() {
        let mut teacher_schedule = Schedule::new();
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let slot = TimeSlot::new(start_time, 3., true);
        teacher_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(11, 0, 0);
        let slot = TimeSlot::new(start_time, 1., false);
        teacher_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(12, 0, 0);
        let slot = TimeSlot::new(start_time, 5., true);
        teacher_schedule.add_slot(&slot);

        // student schedule
        let mut student_schedule = Schedule::new();
        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(8, 0, 0);
        let slot = TimeSlot::new(start_time, 1., false);
        student_schedule.add_slot(&slot);

        let start_time = NaiveDate::from_ymd(2022, 06, 16).and_hms(9, 0, 0);
        let slot = TimeSlot::new(start_time, 8., false);
        student_schedule.add_slot(&slot);

        let has_availability = teacher_schedule.check_availability(&student_schedule, 1.);
        assert!(!has_availability);
    }
}
