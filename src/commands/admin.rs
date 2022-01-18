use std::sync::{Arc, RwLock};

use crate::config::{Config, Role};
use crate::commands::{Command, CommandError};
use crate::util::perms::check_permission;

pub struct ReloadCommand;

impl Command for ReloadCommand {
    fn name(&self) -> &'static str {
        "reload"
    }
    fn usage(&self) -> &'static str {
        "N/A"
    }
    fn about(&self) -> &'static str {
        "Reloads the bot's config file"
    }
    fn execute(&self, config: Arc<RwLock<Config>>, role: &Role, _input: String) -> Result<String, CommandError> {
        if check_permission(&config.read().unwrap(), "admin.reload".to_string(), role) {
            let mut c = config.write().unwrap();
            *c = Config::get();
            return Ok("Config reloaded!".to_string());
        } else {
            return Err(CommandError::NoPerms);
        }
    }
}