use serenity::{
	client::{Context, EventHandler},
	model::{channel::Message, gateway::Ready},
};

use crate::{ChannelIdContainer, RconContainer};

pub(crate) struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.is_own(&ctx.cache).await {
			return;
		}

		let (chat_channel_id, rcon) = {
			let data_container = ctx.data.read().await;
			(
				*data_container.get::<ChannelIdContainer>().unwrap(),
				data_container.get::<RconContainer>().unwrap().clone(),
			)
		};

		if message.channel_id != chat_channel_id {
			return;
		}

		let nick = match message.author_nick(ctx.http).await {
			Some(n) => n,
			None => message.author.name.clone(),
		};
		let content = message.content_safe(ctx.cache).await;
		println!("Bridge: D->A ({}): {}", nick, content);

		let mut rcon = rcon.lock().await;
		if let Err(e) = rcon
			.exec(format!("ServerChat (D) {}: {}", nick, content).as_str())
			.await
		{
			println!("RCON: Could not send message: {}", e.to_string());
		}
	}

	async fn ready(&self, _ctx: Context, _ready: Ready) {
		println!("Discord: Connected & waiting for events!");
	}
}
