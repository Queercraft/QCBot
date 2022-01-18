use std::sync::{Arc, RwLock};

use crate::config::{Config, Role};
use crate::commands::{Command, CommandError};
use crate::util::perms::check_permission;
use crate::util::regexresponse::regexresponse;

pub struct RegexCommand;

impl Command for RegexCommand {
    fn name(&self) -> &'static str {
        "regex"
    }
    fn usage(&self) -> &'static str {
        "<sentence>"
    }
    fn about(&self) -> &'static str {
        "Forces a check for a regex match, even if the user bypasses regex"
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "cmd.regex".to_string(), role) {
            if let Some(r) = regexresponse(config, input.to_string()) {
                    return Ok(r.0);
            } else {
                return Ok("No match!".to_string());
            }
        } else {
            return Err(CommandError::NoPerms);
        }
    }
}