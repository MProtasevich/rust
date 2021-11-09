use core::fmt::{ Display, Formatter, Result };
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i8,
    minutes: i8,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let calculated_hours = hours + (minutes / 60) + if minutes % 60 < 0 { -1 } else { 0 };
        let h = calculated_hours.rem_euclid(24);
        let m = minutes.rem_euclid(60);

        Clock { hours: h as i8, minutes: m as i8 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours as i32, self.minutes as i32 + minutes)
    }
}
