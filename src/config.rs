use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub prefix: String,
    pub commands: HashMap<String, String>,
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
            commands: HashMap::new(),
        };
        
        config.bot_token = "XXXXXX".to_string();
        config.prefix = "!".to_string();
        config.commands.insert("ping".to_string(), "Pong!".to_string());
        config.commands.insert("pong".to_string(), "Pong!".to_string());

        let out = to_string(&config).expect("Failed to convert to TOML format");
        write(path, &out).expect("Failed to write config.toml");
        out
    }

    pub fn get() -> Self {
        let path = PathBuf::from("./config.toml");
        let mut file = String::new();
        
        match read_to_string(path) {
            Ok(f) => {
                file = f;
            },
            Err(e) => {
                println!("{}\nNo config file found! Creating a new file, please configure it with your bot token", e);
                file = Config::create();
            },
        }

        from_str(&file).expect("Failed to parse TOML")
    }
}