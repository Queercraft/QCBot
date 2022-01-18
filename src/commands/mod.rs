use std::sync::{Arc, RwLock};
use std::fmt;

pub mod admin;
pub mod minecraft;
pub mod regex;
pub mod temperature;
pub mod timezone;

use crate::config::{Config, Role};

#[derive(Debug)]
pub enum CommandError {
    BadUsage(String),
    InvalidSyntax(String),
    NoPerms,
    NoCommand,
}

impl std::error::Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::BadUsage(_str) => write!(f, "Bad usage"),
            CommandError::InvalidSyntax(_str) => write!(f, "Invalid syntax"),
            CommandError::NoPerms => write!(f, "No permission"),
            CommandError::NoCommand => write!(f, "No such command"),
        }
    }
}

pub trait Command: Sync + Send  {
    fn name(&self) -> &'static str;
    fn usage(&self) -> &'static str;
    fn about(&self) -> &'static str;
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError>;
}
