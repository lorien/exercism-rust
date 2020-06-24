use chrono::{DateTime, Utc, Duration};

const BILLION_OF_SECONDS: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(BILLION_OF_SECONDS)
}
