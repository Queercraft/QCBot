use std::sync::{Arc, RwLock};

use crate::config::{Config, Role};
use crate::commands::{Command, CommandError};
use crate::util::perms::check_permission;

pub struct TemperatureCommand;

impl Command for TemperatureCommand {
    fn name(&self) -> &'static str {
        "temp"
    }
    fn usage(&self) -> &'static str {
        "Usage: <Degrees> <C|F>"
    }
    fn about(&self) -> &'static str {
        "Converts a number of degrees in Celsius to Fahrenheit, or a number of degrees in Fahrenheit to Celsius"
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "cmd.temperature".to_string(), role) {
            // Get all the numbers (and periods) in the message
            let degrees = input.chars().filter(|c| c.is_digit(10) || c == &'.' || c == &'-').collect::<String>();
            // Search for a C or F
            let format = input.to_lowercase().chars().find(|c| c == &'c' || c == &'f');
            // If a format was found
            if let Some(f) = format {
                // If a number was found
                if !degrees.is_empty() {
                    if f == 'f' {
                        // Check if the number is finite
                        if let Some(fahrenheit) = degrees.parse::<f32>().ok() {
                            if fahrenheit.is_finite() {
                                // Perform the calculation
                                let result: f32 = (fahrenheit - 32.0) / 1.8;
                                // Set the output string
                                return Ok(format!("{} in Fahrenheit is {} in Celsius.", fahrenheit, (result * 100.00).round() / 100.0));
                            } else {
                                // If user inputs ridiculously high number
                                return Err(CommandError::BadUsage("I dunno lol".to_string()));
                            }
                        }
                    } else if f == 'c' {
                        if let Some(celsius) = degrees.parse::<f32>().ok() {
                            if celsius.is_finite() {
                                let result: f32 = (celsius * 1.8) + 32.0;
                                return Ok(format!("{} in Celsius is {} in Fahrenheit.", celsius, (result * 100.00).round() / 100.0));
                            } else {
                                return Err(CommandError::BadUsage("I dunno lol".to_string()));
                            }
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
