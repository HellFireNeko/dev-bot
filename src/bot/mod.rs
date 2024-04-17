use log::info;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::is_shutdown_flag_set;

const TOKEN: &str = include_str!("../../secrets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        // Handle registration:
        info!("Shard {} ready!", ctx.shard_id.0);
        loop {
            if is_shutdown_flag_set() {
                ctx.shard.shutdown_clean();
                break;
            }
        }
    }
}

pub async fn execute() {
    info!("Hello from the bot thread!");
    
    let intents =
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::DIRECT_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&TOKEN, intents)
        .event_handler(Handler).await
        .expect("Err creating client");
    
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {why:?}");
    }
}