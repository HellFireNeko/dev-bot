use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::set_shutdown_flag;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction.create_response(
        &ctx.http, 
        CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())
    ).await?;
    if interaction.user.id != 248835673669369856 {
        interaction.edit_response(
            &ctx.http, 
            EditInteractionResponse::new().content("Sorry, but you do not have permission to execute this command!")
        ).await?;
    } else {
        interaction.edit_response(
            &ctx.http, 
            EditInteractionResponse::new().content("Okay, shutting down gracefully!")
        ).await?;
        set_shutdown_flag();
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("shutdown")
        .description("Stops the bot, and everything related to it in its tracks (WARNING DATALOSS LIKELY)")
}