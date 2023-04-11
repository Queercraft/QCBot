use std::sync::{Arc, RwLock};

use crate::config::{Config, Role};
use crate::commands::{Command, CommandError};
use crate::util::perms::check_permission;

pub struct McItemsCommand;

impl Command for McItemsCommand {
    fn name(&self) -> &'static str {
        "mcitems"
    }
    fn usage(&self) -> &'static str {
        "Usage: <number of stacks>"
    }
    fn about(&self) -> &'static str {
        "Converts a given amount of Minecraft stacks (64) to the item count."
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "cmd.mcitems".to_string(), role) {
            // Get all the numbers (and periods) in the message
            let stacks = input.chars().filter(|c| c.is_digit(10) || c == &'.').collect::<String>();
            // Check if numbers were found
            if !stacks.is_empty() {
                // Try to parse into float
                if let Some(s) = stacks.parse::<f32>().ok() {
                    if s.is_finite() {
                        // Multiply by 32
                        let items: f32 = s * 64.0;
                        // Format string
                        return Ok(format!("{} stack{} break{} down into {:.0} item{}",
                        // Replace stack count
                            s,
                        // Make stacks plural if needed
                        if s == 1.0 { "" } else { "s" },
                        // Make verb plural if needed
                        if s != 1.0 { "" } else { "s" },
                        // Replace item output
                        items,
                        // Make items plural if needed
                        if items == 1.0 { "" } else { "s"}));
                    } else {
                        // If user inputs ridiculously high number
                        return Err(CommandError::BadUsage("I dunno lol".to_string()));
                    }
                }
            }
            return Err(CommandError::InvalidSyntax(self.usage().to_string()));
        } else {
            return Err(CommandError::NoPerms)
        }
    }
}

pub struct McStacksCommand;

impl Command for McStacksCommand {
    fn name(&self) -> &'static str {
        "mcstacks"
    }
    fn usage(&self) -> &'static str {
        "Usage: <number of items>"
    }
    fn about(&self) -> &'static str {
        "Converts a given amount of Minecraft items to how many stacks (64) they make up, with the remainder."
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "cmd.mcstacks".to_string(), role) {
            // Get all the numbers in the message
            let items = input.chars().filter(|c| c.is_digit(10)).collect::<String>();
            if !items.is_empty() {
                if let Some(items) = items.parse::<i32>().ok() {
                    // Get the amount of stacks
                    let stacks: i32 = items / 64;
                    // Get the remainder
                    let left: i32 = items % 64;
                    // Format string
                    return Ok(format!("{} item{} break{} down into {} stack{} with {} item{} left over", 
                    // Replace item count
                    items,
                    // Make items plural if needed
                    if items == 1 { "" } else { "s" },
                    // Make verb plural if needed
                    if items != 1 { "" } else { "s" },
                    // Replace stacks count
                    stacks,
                    // Make stacks plural if needed
                    if stacks == 1 { "" } else { "s" },
                    // Replace remainder
                    left,
                    // Make items plural if needed
                    if left == 1 { "" } else { "s" }));
                }
            }
            return Err(CommandError::InvalidSyntax(self.usage().to_string()));
        } else {
            return Err(CommandError::NoPerms);
        }
    }
}

pub struct McShulkersCommand;

impl Command for McShulkersCommand {
    fn name(&self) -> &'static str {
        "mcshulkers"
    }
    fn usage(&self) -> &'static str {
        "Usage: <number of items>"
    }
    fn about(&self) -> &'static str {
        "Converts a given amount of Minecraft items to how many shulkers, stacks. and remaining items they make up."
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if !check_permission(&config.read().unwrap(), "cmd.mcstacks".to_string(), role) {
            return Err(CommandError::NoPerms);
        }
 
        // Get all the numbers in the message
        let items = input.chars().filter(|c| c.is_digit(10)).collect::<String>();
        if !items.is_empty() {
            if let Some(items) = items.parse::<i32>().ok() {
                let shulkers: i32 = items / 1728;
                let stacks: i32 = (items % 1728) / 64;
                let left: i32 = (items % 1728) % 64;

                return Ok(format!(
                    "{} item{} break{} down into {} shulker{} plus {} stack{} and {} item{} left over", 
                    items,
                    pluralize(items),
                    third_person_ending(items),
                    shulkers,
                    pluralize(shulkers),
                    stacks,
                    pluralize(stacks),
                    left,
                    pluralize(left)));
            }
        }
        return Err(CommandError::InvalidSyntax(self.usage().to_string()));
    }
}

fn pluralize(input: i32) -> &'static str {
    return if input == 1 { "" } else { "s" };
}

fn third_person_ending(input: i32) -> &'static str {
    return if input != 1 { "" } else { "s" };
}
