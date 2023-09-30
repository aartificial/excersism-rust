use std::fmt::{Display, Formatter};

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = Self::hours_minutes_to_minutes(hours, minutes);
        Clock {
            minutes: minutes%HOUR,
            hours: minutes/HOUR,
        }
    }

    fn hours_minutes_to_minutes(hours: i32, minutes: i32) -> i32 {
        (((hours * HOUR + minutes) % DAY) + DAY) % DAY
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
