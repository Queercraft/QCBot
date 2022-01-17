use crate::commands::command::{Command, CommandError};

pub struct McItemsCommand;

impl Command for McItemsCommand {
    fn name(&self) -> &'static str {
        "mcitems"
    }
    fn execute(&self, input: String) -> Result<String, CommandError> {
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
        return Err(CommandError::InvalidSyntax("Usage: <number of stacks>".to_string()));
    }
}

pub struct McStacksCommand;

impl Command for McStacksCommand {
    fn name(&self) -> &'static str {
        "mcstacks"
    }
    fn execute(&self, input: String) -> Result<String, CommandError> {
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
        return Err(CommandError::InvalidSyntax("Usage: <number of items>".to_string()));    }
}

