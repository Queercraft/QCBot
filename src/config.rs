use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Role {
    pub id: u64,
    pub webhook_regex: String,
    pub inherit: String,
    pub perms: Vec<String>,
}

impl Default for Role {
    // Create default function called by serde if valuies are missing from config
    fn default() -> Self {
        Self {
            id: 0,
            webhook_regex: "".to_string(),
            inherit: "".to_string(),
            perms: Vec::from([
                "cmd.regex".to_string(),
                "cmd.mcstacks".to_string(),
                "cmd.mcitems".to_string(),
                "cmd.mcshulkers".to_string(),
                "cmd.mcunshulker".to_string(),
                "cmd.temperature".to_string(),
                "cmd.timezone".to_string(),
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
    pub responses: BTreeMap<String, String>,
    pub regex_responses: BTreeMap<String, String>,
    pub aliases: BTreeMap<String, Vec<String>>,
    pub roles: BTreeMap<String, Role>,
}

impl Default for Config {
    // Create default function called by serde if values are missing from config
    fn default() -> Self {
        let admin = Role {
            id: 123456781234567812,
            webhook_regex: "\\[Admin\\].*".to_string(),
            inherit: "default".to_string(),
            perms: Vec::from([
                "admin.reload".to_string(),
                "bypass.regex".to_string(),
                "bypass.cooldown".to_string(),
            ]),
        };
        Self {
            bot_token: "XXXXXX".to_string(),
            prefix: "!".to_string(),
            trim_regex: "".to_string(),
            command_cooldown: 15,
            regex_response_cooldown: 45,
            enabled_utils: Vec::from([
                "regex".to_string(),
                "mcstacks".to_string(),
                "mcitems".to_string(),
                "mcshulkers".to_string(),
                "mcunshulker".to_string(),
                "temperature".to_string(),
                "timezone".to_string(),
            ]),
            responses: BTreeMap::from([
                ("ping".to_string(), "Pong!".to_string()),
                ("pong".to_string(), "Ping!".to_string()),
            ]),
            regex_responses: BTreeMap::from([
                ("is the bot (?:here|on|alive|working)".to_string(), "Nope, definitely not".to_string()),
            ]),
            aliases: BTreeMap::from([
                ("ping".to_string(), Vec::from(["p".to_string(), "test".to_string()])),
            ]),
            roles: BTreeMap::from([
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
