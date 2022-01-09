use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    macros::group,
};

#[macro_use]
extern crate lazy_static;

mod config;

use config::CONFIG;


#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(&CONFIG.prefix) {
            let command = msg.content.strip_prefix(&CONFIG.prefix).unwrap_or_default().split(' ').take(1).next().unwrap_or_default();
            if CONFIG.commands.contains_key(command) {
                match &CONFIG.commands.get(command) {
                    Some(v) => {
                        if let Err(why) = msg.reply(&ctx, &v).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    None => println!("ERROR: Could not send message."),
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
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}