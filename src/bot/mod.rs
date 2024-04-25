mod commands;

use log::{error, info};
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::bot::commands::register;
use crate::{is_shutdown_flag_set, set_shutdown_flag};

const TOKEN: &str = include_str!("../../secrets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        finteraction_create(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        fready(ctx, _ready).await
    }
}

async fn finteraction_create(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => {
            // handle the command
            info!("Recieved a command interaction '{}' from 'U:{}' 'UUID:{}'", command.data.name, command.user.name, command.user.id);

            let content: Option<String> = match command.data.name.as_str() {
                "shutdown" => {
                    if let Err(why) = commands::shutdown::run(&ctx, &command).await {
                        error!("Something went wrong with execution of commands! {why:?}");
                        set_shutdown_flag();
                    }
                    None
                }
                _ => Some("Not implemented yet".into())
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to slash command: {why}");
                }
            }
        }
        Interaction::Component(component) => {
            // handle the component
            info!("Recieved a component interaction '{}' from 'U:{}' 'UUID:{}'", component.data.custom_id, component.user.name, component.user.id);
        }
        Interaction::Modal(modal) => {
            // handle the modal
            info!("Recieved a modal interaction '{}' from 'U:{}' 'UUID:{}'", modal.data.custom_id, modal.user.name, modal.user.id);
        }
        _ => {
            // nothing to do
        }
    }
}

async fn fready(ctx: Context, _ready: Ready) {
    // Handle registration:
    info!("Shard {} ready!", ctx.shard_id.0);
    
    register(&ctx).await;

    loop {
        if is_shutdown_flag_set() {
            ctx.shard.shutdown_clean();
            break;
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