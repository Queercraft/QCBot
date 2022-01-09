use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub bot_token: String,
    pub prefix: String,
    pub response_cooldown: u64,
    pub regex_response_cooldown: u64,
    pub responses: HashMap<String, String>,
    pub regex_responses: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bot_token: "XXXXXX".to_string(),
            prefix: "!".to_string(),
            response_cooldown: 15,
            regex_response_cooldown: 45,
            responses: HashMap::from([
                ("ping".to_string(), "Pong!".to_string()),
                ("pong".to_string(), "Ping!".to_string()),
            ]),
            regex_responses: HashMap::from([
                ("is the bot (?:here|on|alive|working)".to_string(), "Nope, definitely not".to_string()),
            ]),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::get();
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
            Err(e) => {
                println!("{}\nNo config file found! Creating a new file, please configure it with your bot token", e);
                Config::create()
            },
        };
        
        let conf = from_str(&file).expect("Failed to parse TOML");
        write(path, to_string(&conf).unwrap()).expect("Failed to write config.toml");
        conf
    }
}