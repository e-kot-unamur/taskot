use chrono::{prelude::*, Duration};

/// Fetches numbered environment variables with a given prefix.
///
/// # Example
///
/// ```
/// std::env::set_var("THING_0", "hi");
/// std::env::set_var("THING_1", "hello");
/// let vars = taskot::prefixed_vars("THING");
/// assert_eq!(vars, ["hi", "hello"]);
/// ```
pub fn prefixed_vars(prefix: &str) -> Vec<String> {
    let mut vars = vec![];
    let mut index = 0;
    loop {
        let name = format!("{}_{}", prefix, index);
        index += 1;
        if let Ok(var) = std::env::var(name) {
            vars.push(var);
        } else {
            return vars;
        }
    }
}

/// Returns the number of whole weeks elapsed since Mon, 28 Dec 1969 08:30:00.
pub fn week_number<Tz: TimeZone>(now: DateTime<Tz>) -> i64 {
    // Epoch time actually starts on Thu, 1 Jan 1970 00:00:00.
    // Therefore, we first add 3 days to our timestamp.
    (now + Duration::days(3) - Duration::hours(8) - Duration::minutes(30)).timestamp()
        / Duration::weeks(1).num_seconds()
}

/// Returns the remaining duration until next Monday at 08:30:00.
pub fn until_monday_08h30<Tz: TimeZone>(now: DateTime<Tz>) -> Duration {
    let today_08h30 = now.date().and_hms(8, 30, 0);

    let days_to_wait = if today_08h30.weekday() == Weekday::Mon {
        if now < today_08h30 {
            Duration::zero()
        } else {
            Duration::weeks(1)
        }
    } else {
        Duration::days(7 - today_08h30.weekday().num_days_from_monday() as i64)
    };

    let monday_08h30 = today_08h30 + days_to_wait;
    monday_08h30 - now
}