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
use config::Config;

mod commands;

use commands::command::{Command, CommandError};
use commands::minecraft::{McItemsCommand, McStacksCommand};
use commands::temperature::TemperatureCommand;
use commands::timezone::TimezoneCommand;

#[group]
struct General;

// Define hashmaps for cooldowns
struct Handler {
    // The config for the bot
    config: Config,
    // HashMap of commands by name and the function
    registered_commands: Arc<RwLock<HashMap<String, Box<dyn Command>>>>,
    // HashMap of command cooldowns
    command_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
    // HashMap of Regex cooldowns
    regex_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Handler {
    // Add command
    pub fn register_command(&mut self, command: Box<dyn Command>) {
        let name = &command.name();
        self.registered_commands.write().unwrap().insert(name.to_string(), command);
    }
    // Create handler with data
    pub fn new() -> Handler {
        let mut handler = Handler {
            config: Config::get(),
            registered_commands: Arc::new(RwLock::new(HashMap::new())),
            command_cooldowns: Arc::new(RwLock::new(HashMap::new())),
            regex_cooldowns: Arc::new(RwLock::new(HashMap::new()))
        };
        // TODO make this cleaner
        // Register commands
        Self::register_command(&mut handler, Box::new(McItemsCommand));
        Self::register_command(&mut handler, Box::new(McStacksCommand));
        Self::register_command(&mut handler, Box::new(TemperatureCommand));
        Self::register_command(&mut handler, Box::new(TimezoneCommand));
        // Return handler
        handler
    }

}

// Check permission with function
fn check_permission(config: &Config, command: String, role: &Role) -> bool {
    if role.perms.contains(&command) {
        return true;
    } else {
        if !role.inherit.is_empty() {
            return check_permission(config, command, &config.roles.get(&role.inherit).unwrap());
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
        if !msg.is_own(&ctx).await  {
            // Get users permission group
            let mut r = String::new(); 
            // If the message is not from a webhook
            if msg.webhook_id.is_none() {
                if let Some(member) = &msg.member {
                    for memberrole in &member.roles {
                        if let Some((name, _role)) = self.config.roles.iter().find(|(_name, role)|
                        &role.id == memberrole.as_u64()) {
                                r = name.to_string();
                                break;
                        }
                    }
                }
            // If the message is from a webhook
            } else {    
                for (name, role) in &self.config.roles {
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

            // Set role
            let role = self.config.roles.get(&r).unwrap(); 

            // Trim specified regex from messages
            let content = Regex::new(&self.config.trim_regex).unwrap().replace_all(&msg.content, "");

            // Check if the message starts with the prefix
            if content.starts_with(&self.config.prefix) {
                // Get the word after the prefix
                let mut command = content.strip_prefix(&self.config.prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default();

                // Check if the command is set as an alias everywhere
                // If so, redefine the command to what the alias is for
                for (cmd, aliases) in &self.config.aliases {
                    for a in aliases {
                        if command == a {
                            command = &cmd
                        }
                    }
                }
                
                // Create string that will be the output
                let mut output = String::new();
                
                // Check if command is on cooldown or user bypasses cooldown 
                if role.bypass_command_cooldown == true || !self.command_cooldowns.read().unwrap().contains_key(command) || 
                self.command_cooldowns.read().unwrap().get(command).unwrap().elapsed().as_secs() > self.config.command_cooldown { 

                // Check if the command is a canned response
                    match self.config.responses.get(command) {
                        Some(v) => {
                            // Check if the command is in the cooldown list or has been used more than the cooldown time in seconds ago. If both are false, set reply
                            if self.config.responses_allowed_default || check_permission(&self.config, command.to_string(), role) {
                                // Set cooldown
                                self.command_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                                output = v.to_string();
                                output = output
                                .replace("%username%", &msg.author.name)
                                .replace("%content%", &content.split_once(' ').unwrap_or_default().1.to_string())
                            } else {
                                if let Err(why) = msg.react(&ctx, '❌').await {
                                    println!("Error reacting to message: {:?}", why);
                                };
                            }
                        }
                        None => {
                            // Send to command executor
                            if self.config.enabled_utils.contains(&command.to_string()) {
                                if check_permission(&self.config, command.to_string(), role) {
                                    match self.registered_commands.read().unwrap().get(command) {
                                        Some(cmd) => {
                                            output = match cmd.execute(content.split_once(' ').unwrap_or_default().1.to_string()) {
                                                Ok(o) => {
                                                    // Set cooldown and set reply
                                                    self.command_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                                                    o
                                                },
                                                Err(CommandError::BadUsage(o)) => {
                                                    o
                                                }
                                                Err(CommandError::InvalidSyntax(o)) => {
                                                    o
                                                }
                                            }
                                        }
                                        // Do nothing if the command isn't found
                                        None => ()
                                    }
                                    // output = commands(command.to_string(), 
                                    // content.split_once(' ').unwrap_or_default().1.to_string());
                                } else {
                                    if let Err(why) = msg.react(&ctx, '❌').await {
                                        println!("Error reacting to message: {:?}", why);
                                    };
                                }
                            }
                        }
                    }

                    // Send message if there was an output
                    if !output.is_empty() {
                        if let Err(why) = msg.reply(&ctx, output).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                // React with hourglass emote if command is on cooldown
                } else {
                    if let Err(why) = msg.react(&ctx, '⏳').await {
                        println!("Error reacting to message: {:?}", why);
                    };
                }

                    
            } else {
                if role.bypass_regex == false {
                    // Check if the message matches a defined regex
                    for (regex, response) in &self.config.regex_responses {
                        if Regex::new(&regex.to_lowercase()).unwrap().is_match(&content.to_lowercase()) {
                            // Check if the regex is in the cooldown list or has been used more than the cooldown time in seconds ago. If both are false, send reply
                            if !self.regex_cooldowns.read().unwrap().contains_key(regex) ||
                            self.regex_cooldowns.read().unwrap().get(regex).unwrap().elapsed().as_secs() > self.config.regex_response_cooldown {
                                if let Err(why) = msg.reply(&ctx, response).await {
                                    println!("Error sending message: {:?}", why);
                                }
                                self.regex_cooldowns.write().unwrap().insert(regex.to_string(), Instant::now());
                            }
                            // Break loop
                            break;
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
    let mut client = Client::builder(Config::get().bot_token)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}