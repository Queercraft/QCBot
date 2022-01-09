use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    macros::group,
};
use regex::Regex;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;
extern crate serde;

mod config;

use config::CONFIG;

#[group]
struct General;

struct Handler {
    response_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
    regex_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            response_cooldowns: Arc::new(RwLock::new(HashMap::new())),
            regex_cooldowns: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(&CONFIG.prefix) {
            let command = msg.content.strip_prefix(&CONFIG.prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default();
            match &CONFIG.responses.get(command) {
                Some(v) => {
                    if !self.response_cooldowns.read().unwrap().contains_key(command) || 
                    self.response_cooldowns.read().unwrap().get(command).unwrap().elapsed().as_secs() > CONFIG.response_cooldown {
                        if let Err(why) = msg.reply(&ctx, &v).await {
                            println!("Error sending message: {:?}", why);
                        }    
                        self.response_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                    }
                }
                None => (),
            }    
        } else {
            for (regex, response) in &CONFIG.regex_responses {
                if Regex::new(regex).unwrap().is_match(&msg.content) {
                    if !self.regex_cooldowns.read().unwrap().contains_key(regex) ||
                    self.regex_cooldowns.read().unwrap().get(regex).unwrap().elapsed().as_secs() > CONFIG.regex_response_cooldown {
                        if let Err(why) = msg.reply(&ctx, response).await {
                            println!("Error sending message: {:?}", why);
                        }
                        self.regex_cooldowns.write().unwrap().insert(regex.to_string(), Instant::now());
                    }
                }
            }
        } 
    }
}

#[tokio::main]
async fn main() {

    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP);

    // Login with a bot token from the config file
    let mut client = Client::builder(&CONFIG.bot_token)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}