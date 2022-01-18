use std::sync::{Arc, RwLock};
use std::str::FromStr;

use chrono::{DateTime, NaiveTime, Utc};
use chrono_tz::Tz;

use crate::config::{Config, Role};
use crate::commands::{Command, CommandError};
use crate::util::perms::check_permission;

// Capitalise if the entire string if it doesn't contain a /, if it does, capitalise the first character and everything after a / or _
fn capitalise_timezone(input_str: String) -> String {
    if input_str.contains("/") {
        let mut output_vec: Vec<char> = Vec::new();
        for (i, c) in input_str.chars().enumerate() {
            if i == 0 {
                output_vec.push(c.to_ascii_uppercase());
                output_vec.push(input_str.chars().nth(i + 1).unwrap_or(' '))
            } else if c == '/' || c == '_' {
                output_vec.push(input_str.chars().nth(i + 1).unwrap_or(' ').to_ascii_uppercase())
            } else if i < input_str.len() - 1 {
                output_vec.push(input_str.chars().nth(i + 1).unwrap())
            }
        }
        return output_vec.into_iter().collect::<String>();
    } else {
        return input_str.to_uppercase();
    }
}

pub struct TimezoneCommand;

impl Command for TimezoneCommand {
    fn name(&self) -> &'static str {
        "timezone"
    }
    fn usage(&self) -> &'static str {
        "Usage: <HH:MM(AM/PM)> <TZ> (TZ)"
    }
    fn about(&self) -> &'static str {
        "Converts a given time and timezone to the unix timestamp, which can be embedded with Discord to the user's local time.
        Can optionally specify a second timezone to do a direct conversion. This command also links to https://time.is for more information"
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "cmd.timezone".to_string(), role) {
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
            let mut tz_str = input.to_ascii_lowercase().split(' ').skip(1).next().unwrap_or_default().
            chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
            // Attempt to get a second timezone

            let mut to_tz_str = input.to_ascii_lowercase().split(' ').skip(2).next().unwrap_or_default().
            chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
            // Check if a timezone was found
            if !tz_str.is_empty() {
                // Use function to set proper capitalisation before getting from enum
                tz_str = capitalise_timezone(tz_str);
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
                        if !to_tz_str.is_empty() {
                            // Use function to set proper capitalisation before getting from enum
                            to_tz_str = capitalise_timezone(to_tz_str);
                            // Convert the two timezones
                            if let Ok(to_tz) = Tz::from_str(&to_tz_str) {
                                return Ok(format!("{} {} today in {} is {}, more at https://time.is/compare/{}_in_{}/{}",
                                ntime.format("%H:%M").to_string(), tz.name(), to_tz.name(), tz_today.with_timezone(&to_tz).format("%H:%M"), 
                                ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap(), to_tz.name().split('/').last().unwrap()));
                            }
                        // If only one timezone was specified, use Discord's Unix Timestamp embed and link to a time.is table using an unspecified timezone
                        } else {
                            return Ok(format!("{} {} today in your local timezone is <t:{}>, more at https://time.is/compare/{}_in_{}",
                            ntime.format("%H:%M").to_string(), tz.name(),tz_today.format("%s").to_string() , ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap()));
                        }
                    }
                }
            }
            return Err(CommandError::InvalidSyntax(self.usage().to_string()));
        } else {
            return Err(CommandError::NoPerms);
        }
    }
}
