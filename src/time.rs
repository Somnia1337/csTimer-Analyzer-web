/// Milliseconds in u32.
pub type Milliseconds = u32;

/// Seconds in f32.
pub type Seconds = f32;

/// Milliseconds in a second.
const SEC: Milliseconds = 1_000;

/// Milliseconds in a minute.
const MIN: Milliseconds = 60_000;

/// Milliseconds in an hour.
const HOUR: Milliseconds = 3_600_000;

/// Formats a time type into a
/// human-readable string.
pub trait HumanReadable {
    /// The human-readable string of the type.
    fn to_readable_string(&self) -> String;
}

impl HumanReadable for Milliseconds {
    fn to_readable_string(&self) -> String {
        let t = *self;

        if t < SEC {
            return format!("0.{:03}", t);
        } else if t < SEC * 10 {
            return format!("{}.{:03}", t / SEC, t % SEC);
        }

        let secs_and_millis = format!("{:02}.{:03}", (t % MIN) / SEC, t % SEC);

        if t < MIN {
            secs_and_millis
        } else if t < HOUR {
            let mins = t / MIN;

            format!("{}:{}", mins, secs_and_millis)
        } else {
            let hours = t / HOUR;
            let mins = (t % HOUR) / MIN;

            format!("{}:{}:{}", hours, mins, secs_and_millis)
        }
    }
}

impl HumanReadable for Seconds {
    fn to_readable_string(&self) -> String {
        let millis = (*self * 1000.0) as Milliseconds;

        millis.to_readable_string()
    }
}
