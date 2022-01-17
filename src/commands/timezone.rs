use chrono::{DateTime, NaiveTime, Utc};
use chrono_tz::Tz;

use std::str::FromStr;

use crate::commands::command::{Command, CommandError};

pub struct TimezoneCommand;

impl Command for TimezoneCommand {
    fn name(&self) -> &'static str {
        "timezone"
    }
    fn execute(&self, input: String) -> Result<String, CommandError> {
        // Get all characters matching 0-9 and :
        let mut time_str = input.split(' ').take(1).next().unwrap_or_default().to_uppercase()
        .chars().filter(|c| c.is_digit(10) || c == &':').collect::<String>();
        // Attempt to get the AM/PM string
        let meridiem_str = input.split(' ').take(1).next().unwrap_or_default().to_uppercase()
        .chars().filter(|c| c == &'A' || c == &'P' || c == &'M').collect::<String>();
        // Check if the string specifies minutes, if not, assume 00
        if !time_str.contains(":") {
            time_str.push_str(":00");
        }
        // Merge the time string and AM/PM (meridiem)
        time_str.push_str(&meridiem_str);
        // Attempt to get the first timezone
        let tz_str = input.split(' ').skip(1).next().unwrap_or_default().
        chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
        // Attempt to get a second timezone
        let to_tz_str = input.split(' ').skip(2).next().unwrap_or_default().
        chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
        // Check if a timezone was found
        if !tz_str.is_empty() {
            // Check if timezone 1 can be parsed
            if let Ok(tz) = Tz::from_str(&tz_str) {
                // Check if the time is in 12 hour format, if it is, convert it to 24 hours before proceeding
                if let Ok(ntime) = NaiveTime::parse_from_str(&time_str, "%-I:%M%p") {
                    time_str = ntime.format("%H:%M").to_string();
                }
                // Attempt to parse the time as 24 hours
                if let Ok(ntime) = NaiveTime::parse_from_str(&time_str, "%H:%M") {
                    // Get the time in the specified time zone with the current date in that timezone
                    let tz_today: DateTime<Tz> = Utc::today().with_timezone(&tz).and_time(ntime).unwrap();
                    // If a second timezone can be parsed, convert the two timezones and link to a time.is comparison table
                    if let Ok(to_tz) = Tz::from_str(&to_tz_str) {
                        return Ok(format!("{} {} today in {} is {}, more at https://time.is/compare/{}_in_{}/{}",
                        ntime.format("%H:%M").to_string(), tz.name(), to_tz.name(), tz_today.with_timezone(&to_tz).format("%H:%M"), 
                        ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap(), to_tz.name().split('/').last().unwrap()));
                    // If only one timezone was specified, use Discord's Unix Timestamp embed and link to a time.is table using an unspecified timezone
                    } else {
                        return Ok(format!("{} {} today in your local timezone is <t:{}>, more at https://time.is/compare/{}_in_{}",
                        ntime.format("%H:%M").to_string(), tz.name(),tz_today.format("%s").to_string() , ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap()));
                    }
                }
            }
        }
        return Err(CommandError::InvalidSyntax("Usage: <HH:MM(AM/PM)> <TZ> (TZ)".to_string()));
    }
}