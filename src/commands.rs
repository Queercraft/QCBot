pub fn commands(command: String, content: String) -> String {
    let mut output = String::new();
    match command.as_str() {
        "temp" => {
            output = "Temporary Placeholder".to_string();
        }
        _ => (),
    }
    output
}