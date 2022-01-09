use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub prefix: String,
    pub response_cooldown: u64,
    pub regex_response_cooldown: u64,
    pub responses: HashMap<String, String>,
    pub regex_responses: HashMap<String, String>,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::get();

}

impl Config {
    pub fn create() -> String {

        let path = PathBuf::from("./config.toml");
        let mut config = Self { 
            bot_token: String::new(),
            prefix: String::new(),
            response_cooldown: 0,
            regex_response_cooldown: 0,
            responses: HashMap::new(),
            regex_responses: HashMap::new(),
        };
        
        config.bot_token = "XXXXXX".to_string();
        config.prefix = "!".to_string();
        config.response_cooldown = 15;
        config.regex_response_cooldown = 45;
        config.responses.insert("ping".to_string(), "Pong!".to_string());
        config.responses.insert("pong".to_string(), "Pong!".to_string());
        config.regex_responses.insert("is the bot (?:here|on|alive|working)".to_string(), "Nope, definitely not".to_string());

        let out = to_string(&config).expect("Failed to convert to TOML format");
        write(path, &out).expect("Failed to write config.toml");
        out
    }

    pub fn get() -> Self {
        let path = PathBuf::from("./config.toml");
      
        let file = match read_to_string(path) {
            Ok(f) => {
                f
            },
            Err(e) => {
                println!("{}\nNo config file found! Creating a new file, please configure it with your bot token", e);
                Config::create()
            },
        };

        from_str(&file).expect("Failed to parse TOML")
    }
}