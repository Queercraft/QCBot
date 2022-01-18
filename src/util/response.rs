use std::sync::{Arc, RwLock};

use crate::commands::{CommandError};
use crate::config::Role;
use crate::config::Config;
use crate::util::perms::check_permission;

pub fn response(config: Arc<RwLock<Config>>, role: &Role, command: String) -> Result<String, CommandError> {
    // Check if response is in config
    if let Some(r) = config.read().unwrap().responses.get(&command.to_string()) {
        // Check if the user has permission
        if check_permission(&config.read().unwrap(), format!("response.{}", command), &role) {
            // Return Ok with the response
            return Ok(r.to_string());
        } else {
            // Return permission denied
            return Err(CommandError::NoPerms);
        }
    }
    // Return no command
    return Err(CommandError::NoCommand);
}