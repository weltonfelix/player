/// Returns a string of the time in the format of HH:MM:SS
///
/// # Arguments
/// * `milliseconds` - The time in milliseconds
///
/// # Example
/// ```
/// let time = parse_time(1000);
/// assert_eq!(time, "00:01");
/// ```
pub fn parse_time(milliseconds: i64) -> String {
    let seconds = milliseconds / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    let formatted_seconds = format!("{:02}", seconds % 60);
    let formatted_minutes = format!("{:02}", minutes % 60);
    let formatted_hours = format!("{:02}", hours);

    let mut time = String::new();

    if hours > 0 {
        time.push_str(&format!("{}:", formatted_hours));
    }

    time.push_str(&format!("{}:", formatted_minutes));
    time.push_str(&format!("{}", formatted_seconds));

    return time;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time(1000), "00:01");
        assert_eq!(parse_time(60000), "01:00");
        assert_eq!(parse_time(3600000), "01:00:00");
        assert_eq!(parse_time(3661000), "01:01:01");
    }
}
