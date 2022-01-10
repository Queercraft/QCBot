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

// Define hashmaps for cooldowns
struct Handler {
    response_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
    regex_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
}

// Create hashmaps on the handler
impl Handler {
    pub fn new() -> Handler {
        Handler {
            response_cooldowns: Arc::new(RwLock::new(HashMap::new())),
            regex_cooldowns: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

// Implements functions for events
#[async_trait]
impl EventHandler for Handler {
    // Run on message
    async fn message(&self, ctx: Context, msg: Message) {
        // Get users permission group
        let mut role = String::new(); 
        if let Some(member) = &msg.member {
            for memberrole in &member.roles {
                for (n, r) in &CONFIG.roles {
                    if &r.id == memberrole.as_u64() {
                        role = n.to_string();
                    }
                }
            }
        }
        // Fall back to default role
        if role == "" {
            role = "default".to_string();
        }

        // Check if the message starts with the prefix
        if msg.content.starts_with(&CONFIG.prefix) {
            // Get the word after the prefix
            let mut command = msg.content.strip_prefix(&CONFIG.prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default();
            // Check if the command is set as an alias everywhere
            // If so, redefine the command to what the alias is for
            for (cmd, aliases) in &CONFIG.reponses_aliases {
                for a in aliases {
                    if command == a {
                        command = cmd
                    }
                }
            }

            // Check if the command is a canned response
            match &CONFIG.responses.get(command) {
                Some(v) => {
                    // Check if the command is in the cooldown list or has been used more than the cooldown time in seconds ago. If both are false, send reply
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
            // Check if the message matches a defined regex
            for (regex, response) in &CONFIG.regex_responses {
                if Regex::new(regex).unwrap().is_match(&msg.content) {
                    // Check if the regex is in the cooldown list or has been used more than the cooldown time in seconds ago. If both are false, send reply
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