use serenity::async_trait;
use serenity::prelude::GatewayIntents;
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

mod util;
use util::response::response;
use util::regexresponse::regexresponse;
use util::perms::check_permission;

mod config;
use config::Config;

mod commands;
use commands::{Command, CommandError};
use commands::minecraft::{McItemsCommand, McStacksCommand, McShulkersCommand, McUnshulkerCommand};
use commands::temperature::TemperatureCommand;
use commands::timezone::TimezoneCommand;
use commands::admin::ReloadCommand;
use commands::regex::RegexCommand;

#[group]
struct General;

// Define hashmaps for cooldowns
struct Handler {
    // The config for the bot
    config: Arc<RwLock<Config>>,
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
            config: Arc::new(RwLock::new(Config::get())),
            registered_commands: Arc::new(RwLock::new(HashMap::new())),
            command_cooldowns: Arc::new(RwLock::new(HashMap::new())),
            regex_cooldowns: Arc::new(RwLock::new(HashMap::new()))
        };
        // Register commands
        Self::register_command(&mut handler, Box::new(McItemsCommand));
        Self::register_command(&mut handler, Box::new(McStacksCommand));
        Self::register_command(&mut handler, Box::new(McShulkersCommand));
        Self::register_command(&mut handler, Box::new(McUnshulkerCommand));
        Self::register_command(&mut handler, Box::new(TemperatureCommand));
        Self::register_command(&mut handler, Box::new(TimezoneCommand));
        Self::register_command(&mut handler, Box::new(ReloadCommand));
        Self::register_command(&mut handler, Box::new(RegexCommand));

        // Return handler
        handler
    }

}


// Implements functions for events
#[async_trait]
impl EventHandler for Handler {
    // Run on message
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.is_own(&ctx) {
            // Get users permission group
            let mut r = String::new(); 
            // If the message is not from a webhook
            if msg.webhook_id.is_none() {
                if let Some(member) = &msg.member {
                    for memberrole in &member.roles {
                        if let Some((name, _role)) = self.config.read().unwrap().roles.iter().find(|(_name, role)|
                        &role.id == memberrole.as_u64()) {
                                r = name.to_string();
                                break;
                        }
                    }
                }
            // If the message is from a webhook
            } else {    
                for (name, role) in &self.config.read().unwrap().roles {
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
            let role = self.config.read().unwrap().roles.get(&r).unwrap().clone(); 

            // Trim specified regex from messages
            let content = Regex::new(&self.config.read().unwrap().trim_regex).unwrap().replace_all(&msg.content, "");

            // Create string that will be the output
            let mut reply = String::new();
            // Create list of emotes that the bot will react with
            let mut emotes: Vec<char> = Vec::new();

            // Check if the message starts with the prefix, if so, execute commands
            if content.starts_with(&self.config.read().unwrap().prefix) {
                // Get the word after the prefix
                let mut command = content.strip_prefix(&self.config.read().unwrap().prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default().to_string().to_lowercase();

                // Check if the command is set as an alias
                // If so, redefine the command to what the alias is for
                for (cmd, aliases) in &self.config.read().unwrap().aliases {
                    for a in aliases {
                        if &command == a {
                            command = cmd.to_string()
                        }
                    }
                }
                
                // Check if command isn't on cooldown or user bypasses cooldown 
                if check_permission(&self.config.read().unwrap(), "bypass.cooldown".to_string(), &role) || !self.command_cooldowns.read().unwrap().contains_key(&command.to_string()) || 
                self.command_cooldowns.read().unwrap().get(&command.to_string()).unwrap().elapsed().as_secs() > self.config.read().unwrap().command_cooldown {
                    // Check if the command has a response
                    match response(self.config.clone(), &role, command.to_string()) {
                        // If the command has a response, set the reply and cooldown
                        Ok(r) => {
                            reply = r
                            .replace("%username%", &msg.author.name)
                            .replace("%content%", &content.split_once(' ').unwrap_or_default().1.to_string());

                            self.command_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                        },
                        // If permission is denied react with an emote
                        Err(CommandError::NoPerms) => {
                            emotes.push('❌');
                        },
                        // If there is no response matching, check if there's a utility command for it
                        Err(CommandError::NoCommand) => {
                            if let Some(cmd) = self.registered_commands.read().unwrap().get(&command.to_string()) {
                                match cmd.execute(self.config.clone(), &role, content.split_once(' ').unwrap_or_default().1.to_string()) {
                                    // If the command was successful, set the reply and cooldown
                                    Ok(o) => {
                                        // Set cooldown and set reply
                                        self.command_cooldowns.write().unwrap().insert(command.to_string(), Instant::now());
                                        reply = o;
                                    },
                                    // If the input was invalid react with an emote
                                    Err(CommandError::BadUsage(o)) => {
                                        reply = o;
                                        emotes.push('💢');
                                    },
                                    // If the syntax was invalid react with an emote
                                    Err(CommandError::InvalidSyntax(o)) => {
                                        reply = o;
                                        emotes.push('❔');
                                    },
                                    // If permission is denied react with an emote
                                    Err(CommandError::NoPerms) => {
                                        emotes.push('❌');
                                    },
                                    _ => ()
                                }
                            }
        
                        }
                        _ => ()
                        
                    } 

                // Add hourglass emote if command is on cooldown
                } else {
                    emotes.push('⏳');
                }

            } else if !check_permission(&self.config.read().unwrap(), "bypass.regex".to_string(), &role) {
                if let Some(r) = regexresponse(self.config.clone(), content.to_string()) {
                    if !self.regex_cooldowns.read().unwrap().contains_key(&r.1) ||
                    self.regex_cooldowns.read().unwrap().get(&r.1).unwrap().elapsed().as_secs() > self.config.read().unwrap().regex_response_cooldown {
                        reply = r.0;
                        self.regex_cooldowns.write().unwrap().insert(r.1.to_string(), Instant::now());    
                    }
                }
            }

            if !reply.is_empty() {
                if let Err(why) = msg.reply(&ctx, reply).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            for e in emotes {
                if let Err(why) = msg.react(&ctx, e).await {
                    println!("Error reacting to message: {:?}", why);
                };

            }

        }
    }
}

#[tokio::main]
async fn main() {

    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP);

    // Login with a bot token from the config file
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(Config::get().bot_token, intents)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}
