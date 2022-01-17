use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Role {
    pub id: u64,
    pub webhook_regex: String,
    pub inherit: String,
    pub bypass_regex: bool,
    pub bypass_command_cooldown: bool,
    pub perms: Vec<String>,
}

impl Default for Role {
    // Create default function called by serde if valuies are missing from config
    fn default() -> Self {
        Self {
            id: 0,
            webhook_regex: "".to_string(),
            inherit: "".to_string(),
            bypass_regex: false,
            bypass_command_cooldown: false,
            perms: Vec::from([
                "8ball".to_string(),
                "mcstacks".to_string(),
                "mcitems".to_string(),
                "temp".to_string(),
                "timezone".to_string(),
                "ping".to_string(),
                ]),
        }
    }
}

// Struct of all the config options
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub bot_token: String,
    pub prefix: String,
    pub trim_regex: String,
    pub command_cooldown: u64,
    pub regex_response_cooldown: u64,
    pub enabled_utils: Vec<String>,
    pub responses_allowed_default: bool,
    pub responses: HashMap<String, String>,
    pub regex_responses: HashMap<String, String>,
    pub aliases: HashMap<String, Vec<String>>,
    pub roles: HashMap<String, Role>,
}

impl Default for Config {
    // Create default function called by serde if values are missing from config
    fn default() -> Self {
        let admin = Role {
            id: 123456781234567812,
            webhook_regex: "\\[Admin\\].*".to_string(),
            inherit: "default".to_string(),
            bypass_regex: true,
            bypass_command_cooldown: true,
            perms: Vec::from(["pong".to_string()]),
        };
        Self {
            bot_token: "XXXXXX".to_string(),
            prefix: "!".to_string(),
            trim_regex: "".to_string(),
            command_cooldown: 15,
            regex_response_cooldown: 45,
            enabled_utils: Vec::from([
                "8ball".to_string(),
                "mcstacks".to_string(),
                "mcitems".to_string(),
                "temp".to_string(),
                "timezone".to_string(),
            ]),
            responses_allowed_default: true,
            responses: HashMap::from([
                ("ping".to_string(), "Pong!".to_string()),
                ("pong".to_string(), "Ping!".to_string()),
            ]),
            regex_responses: HashMap::from([
                ("is the bot (?:here|on|alive|working)".to_string(), "Nope, definitely not".to_string()),
            ]),
            aliases: HashMap::from([
                ("ping".to_string(), Vec::from(["p".to_string(), "test".to_string()])),
            ]),
            roles: HashMap::from([
                ("default".to_string(), Role::default()),
                ("admin".to_string(), admin),
            ])
        }
    }
}

impl Config {
    pub fn create() -> String {
        let path = PathBuf::from("./config.toml");
        let config = Self { ..Default::default() };
        
        let out = to_string(&config).expect("Failed to convert to TOML format");
        write(path, &out).expect("Failed to write config.toml");
        out
    }

    pub fn get() -> Self {
        let path = PathBuf::from("./config.toml");
        let file = match read_to_string(&path) {
            Ok(f) => {
                f
            },
            // If file does not exist
            Err(e) => {
                println!("{}\nNo config file found! Creating a new file, please configure it with your bot token", e);
                Config::create()
            },
        };
        
        // Load config from file, injecting defaults if values are missing
        let conf = from_str(&file).expect("Failed to parse TOML");
        // Rewrite config to file, this creates values that were missing, if any
        write(path, to_string(&conf).unwrap()).expect("Failed to write config.toml");
        conf
    }
}