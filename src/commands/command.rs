use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    BadUsage(String),
    InvalidSyntax(String),
}

impl std::error::Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::BadUsage(_str) => write!(f, "Bad usage"),
            CommandError::InvalidSyntax(_str) => write!(f, "Invalid syntax"),
        }
    }
}

pub trait Command: Sync + Send  {
    fn name(&self) -> &'static str;
    fn execute(&self, input: String) -> Result<String, CommandError>;
}

pub struct TestCommand;

impl Command for TestCommand {
    fn name(&self) -> &'static str {
        "test"
    }
    fn execute(&self, _input: String) -> Result<String, CommandError> {
        Ok("Testing!!!".to_string())
    }
}