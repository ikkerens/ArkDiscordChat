use serenity::{client::Context,
               framework::standard::{
                   Args,
                   CommandResult,
                   macros::{
                       command,
                       group,
                   },
               },
               model::channel::Message};

use crate::type_container::RconContainer;

group!({
    name: "general",
    options: {},
    commands: [rcon]
});

#[command]
#[required_permissions("MANAGE_GUILD")]
fn rcon(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if msg.is_own(&ctx.cache) {
        return Ok(());
    }

    let rcon = {
        let data_container = ctx.data.read();
        data_container.get::<RconContainer>().unwrap().clone()
    };

    let response = {
        let mut rcon = rcon.lock().unwrap();
        match rcon.exec(args.rest()) {
            Err(e) => {
                println!("RCON: Could not execute custom command: {}", e.to_string());
                "Something went wrong while executing that command.".to_string()
            }
            Ok(r) => format!("Server response:\n```\n{}\n```", r.trim())
        }
    };

    msg.reply(ctx.http.clone(), response).unwrap();
    Ok(())
}