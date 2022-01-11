pub fn commands(command: String, content: String) -> String {
    let mut output = String::new();
    match command.as_str() {
        // If the command is temp
        "temp" => {
            // Get all the numbers (and periods) in the message
            let degrees = content.chars().filter(|c| c.is_digit(10) || c == &'.' || c == &'-').collect::<String>();
            // Search for a C or F
            let format = content.to_lowercase().chars().find(|c| c == &'c' || c == &'f');
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
                                output = format!("{} in Fahrenheit is {} in Celsius.", fahrenheit, (result * 100.00).round() / 100.0);
                            } else {
                                output = "I dunno lol".to_string();
                            }
                        }
                    } else if f == 'c' {
                        if let Some(celsius) = degrees.parse::<f32>().ok() {
                            if celsius.is_finite() {
                                let result: f32 = (celsius * 1.8) + 32.0;
                                output = format!("{} in Celsius is {} in Fahrenheit.", celsius, (result * 100.00).round() / 100.0);
                            } else {
                                output = "I dunno lol".to_string();
                            }
                        }
                    }
                }
            }
            if output.is_empty() {
                output = "Usage: <Degrees> <C|F>".to_string();
            }
        }
        "mcstacks" => {
            // Get all the numbers in the message
            let items = content.chars().filter(|c| c.is_digit(10)).collect::<String>();
            if !items.is_empty() {
                if let Some(items) = items.parse::<i32>().ok() {
                    // Get the amount of stacks
                    let stacks: i32 = items / 64;
                    // Get the remainder
                    let left: i32 = items % 64;
                    // Format string
                    output = format!("{} item{} break{} down into {} stack{} with {} item{} left over", 
                    items,
                    if items == 1 { "" } else { "s" },
                    if items != 1 { "" } else { "s" },
                    stacks,
                    if stacks == 1 { "" } else { "s" },
                    left,
                    if left == 1 { "" } else { "s" });
                }
            }
            if output.is_empty() {
                output = "Usage: <number of items>".to_string();
            }
        }
        "mcitems" => {
            // Get all the numbers (and periods) in the message
            let stacks = content.chars().filter(|c| c.is_digit(10) || c == &'.').collect::<String>();
            if !stacks.is_empty() {
                if let Some(s) = stacks.parse::<f32>().ok() {
                    if s.is_finite() {
                        // Multiply by 32
                        let items: f32 = s * 64.0;
                        // Format string
                        output = format!("{} stack{} break{} down into {:.0} item{}",
                        // Replace stack count
                         s,
                        // Make stacks plural if needed
                        if s == 1.0 { "" } else { "s" },
                        // Make verb plural if needed
                        if s != 1.0 { "" } else { "s" },
                        // Replace item output
                        items,
                        // Make items plural if needed
                        if items == 1.0 { "" } else { "s"});
                    } else {
                        output = "I dunno lol".to_string();
                    }
                }
            }
            if output.is_empty() {
                output = "Usage: <number of stacks>".to_string();
            }

        }
        _ => (),
    }
    output
}