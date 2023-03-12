use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes;
        let mut hours = if minutes >= 0 {
            hours + minutes / 60
        } else {
            if minutes % 60 == 0 {
                hours + minutes / 60
            } else {
                hours + minutes / 60 - 1
            }
        };

        hours = (hours + 24 * 10000) % 24;
        minutes = (minutes + 60 * 10000) % 60;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = self.minutes + minutes;
        let mut hours = if minutes >= 0 {
            self.hours + minutes / 60
        } else {
            if minutes % 60 == 0 {
                self.hours + minutes / 60
            } else {
                self.hours + minutes / 60 - 1
            }
        };

        hours = (hours + 24 * 10000) % 24;
        minutes = (minutes + 60 * 10000) % 60;
        Self { hours, minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
