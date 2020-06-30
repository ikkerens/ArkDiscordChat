use std::{env, sync::Arc, time::Duration};

use rercon::ReConnection;
use serenity::{framework::StandardFramework, model::id::ChannelId, Client};
use tokio::sync::Mutex;

use crate::{
	start_error::StartError,
	type_container::{ChannelIdContainer, RconContainer},
};

mod ark;
mod discord;
mod rcon_command;
mod start_error;
mod type_container;

#[tokio::main]
async fn main() -> Result<(), StartError> {
	if !envmnt::is_all_exists(&vec!["RCON_HOST", "RCON_PASS", "DISCORD_TOKEN", "DISCORD_CHANNEL"]) {
		return Err(StartError::from("Bridge: Not all of the RCON_HOST, RCON_PASS, DISCORD_TOKEN and DISCORD_CHANNEL environment vars have been set. Exiting".to_string()));
	}

	let channel = ChannelId(env::var("DISCORD_CHANNEL")?.parse::<u64>()?);

	println!("Starting ARK<->Discord chat bridge...");
	println!("RCON: Connecting...");
	let rcon = Arc::new(Mutex::new(
		ReConnection::open(
			env::var("RCON_HOST")?,
			env::var("RCON_PASS")?,
			Some(Duration::from_secs(10)),
		)
		.await?,
	));
	println!("RCON: Connected & reading!");

	println!("Discord: Setting up...");
	let mut discord = Client::new(env::var("DISCORD_TOKEN")?)
		.event_handler(discord::Handler)
		.framework(
			StandardFramework::new()
				.group(&rcon_command::GENERAL_GROUP)
				.configure(|c| c.prefix("!").allow_dm(false)),
		)
		.await?;
	{
		let mut data = discord.data.write().await;
		data.insert::<RconContainer>(rcon.clone());
		data.insert::<ChannelIdContainer>(channel);
	}

	ark::start_loop(rcon, discord.cache_and_http.clone(), channel);

	if let Err(e) = discord.start_autosharded().await {
		return Err(StartError::from(e));
	}

	Ok(())
}
