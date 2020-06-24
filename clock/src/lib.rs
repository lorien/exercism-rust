// Structure to hold clock data (hours and minutes)
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    // Accepts positive/negative hours and minutes
    // Normalizes input arguments so result clock value gets in range
    // from 00:00 to 23:59 (inclusive)
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let mut hours = (total_minutes / 60) % 24;
        let mut minutes = total_minutes % 60;
        // Now it is guaranteed that minutes.abs() < 60
        
        if minutes < 0 {
            hours -= 1;
            minutes = 60 + minutes;
        }
        if hours == 24 {
            hours = 0;
        }
        if hours < 0 {
            hours = 24 + hours;
        }

        Clock{
            hours,
            minutes,
        }
    }

    // Constructs new clock by adding minutes
    // All normalization happens in new() method
    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("Add to {} the {}", self, minutes);
        Clock::new(
            self.hours,
            self.minutes + minutes
        )
    }
}

// powers {:?} debug formatter
impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

// powers std::fmt::Display trait (and also std::string::ToString)
impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
    }
}

// implement == operator
impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

// implement == operator
impl std::cmp::Eq for Clock {}

// Implements std::string::ToString trait
// This trait is automatically implemented for any type which implements the Display trait
//impl std::string::ToString {
//    fn to_string(&self) -> String {}
//}

// Implements std::conver::From<Clock> for String
impl From<Clock> for String {
    fn from(clock: Clock) -> String {
        return clock.to_string();
    }
}
