use serenity::{
	client::Context,
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::channel::Message,
};

use crate::type_container::RconContainer;

#[group]
#[description = "All commands"]
#[commands(rcon)]
struct General;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn rcon(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if msg.is_own(&ctx.cache).await {
		return Ok(());
	}

	let rcon = {
		let data_container = ctx.data.read().await;
		data_container.get::<RconContainer>().unwrap().clone()
	};

	let response = {
		let mut rcon = rcon.lock().await;
		match rcon.exec(args.rest()).await {
			Err(e) => {
				println!("RCON: Could not execute custom command: {}", e.to_string());
				"Something went wrong while executing that command.".to_string()
			}
			Ok(r) => format!("Server response:\n```\n{}\n```", r.trim()),
		}
	};

	msg.reply(ctx.http.clone(), response).await.ok();
	Ok(())
}
