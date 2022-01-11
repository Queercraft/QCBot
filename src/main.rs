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

mod config;

use config::Role;
use config::CONFIG;

mod commands;

use commands::commands;

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

fn check_permission(command: String, role: &Role) -> bool {
    if role.perms.contains(&command) {
        return true;
    } else {
        if !role.inherit.is_empty() {
            return check_permission(command, CONFIG.roles.get(&role.inherit).unwrap());
        } else {
            return false;
        }
    }
}

// Implements functions for events
#[async_trait]
impl EventHandler for Handler {
    // Run on message
    async fn message(&self, ctx: Context, msg: Message) {
        // Get users permission group
        let mut r = String::new(); 
        // If the message is not from a webhook
        if msg.webhook_id.is_none() {
            if let Some(member) = &msg.member {
                for memberrole in &member.roles {
                    if let Some((name, _role)) = CONFIG.roles.iter().find(|(_name, role)|
                    &role.id == memberrole.as_u64()) {
                            r = name.to_string();
                            break;
                    }
                }
            }
        // If the message is from a webhook
        } else {    
            for (name, role) in &CONFIG.roles {
                if !role.webhook_regex.is_empty() &&  Regex::new(&role.webhook_regex).unwrap().is_match(&msg.author.name) {
                    r = name.to_string();
                    break;
                }
            }
            
         };

        // Fall back to default role
        if r == "" {
            r = "default".to_string();
        }
        let role = &CONFIG.roles.get(&r).unwrap(); 

        let content = Regex::new(&CONFIG.trim_regex).unwrap().replace_all(&msg.content, "");

        // Check if the message starts with the prefix
        if content.starts_with(&CONFIG.prefix) {
            // Get the word after the prefix
            let mut command = content.strip_prefix(&CONFIG.prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default();

            // Check if the command is set as an alias everywhere
            // If so, redefine the command to what the alias is for
            for (cmd, aliases) in &CONFIG.reponses_aliases {
                for a in aliases {
                    if command == a {
                        command = cmd
                    }
                }
            }

            let mut output = String::new();
            
            if role.bypass_response_cooldown == true || !self.response_cooldowns.read().unwrap().contains_key(command) || 
            self.response_cooldowns.read().unwrap().get(command).unwrap().elapsed().as_secs() > CONFIG.response_cooldown { 

            // Check if the command is a canned response
                match &CONFIG.responses.get(command) {
                    Some(v) => {
                        // Check if the command is in the cooldown list or has been used more than the cooldown time in seconds ago. If both are false, send reply
                        if CONFIG.responses_allowed_default || check_permission(command.to_string(), role) {
                            output = v.to_string();
                        } else {
                            if let Err(why) = msg.react(&ctx, '❌').await {
                                println!("Error reacting to message: {:?}", why);
                            };
                        }
                    }
                    None => {
                        // Send to command executor
                        if CONFIG.enabled_utils.contains(&command.to_string()) {
                            if check_permission(command.to_string(), role) {
                                output = commands(command.to_string(), 
                                content.split_once(' ').unwrap_or_default().1.to_string());
                            } else {
                                if let Err(why) = msg.react(&ctx, '❌').await {
                                    println!("Error reacting to message: {:?}", why);
                                };
                            }
                        }
                    }
                }

                if !output.is_empty() {
                    self.response_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                    if let Err(why) = msg.reply(&ctx, output).await {
                        println!("Error sending message: {:?}", why);
                    }
                }

            } else {
                if let Err(why) = msg.react(&ctx, '⏳').await {
                    println!("Error reacting to message: {:?}", why);
                };
            }

                
        } else {
            if role.bypass_regex == false {
                // Check if the message matches a defined regex
                for (regex, response) in &CONFIG.regex_responses {
                    if Regex::new(regex).unwrap().is_match(&content) {
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