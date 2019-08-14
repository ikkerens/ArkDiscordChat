use serenity::{client::Context,
               model::channel::Message,
               prelude::EventHandler};

use crate::{ChannelIdContainer, RconContainer};

pub(crate) struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.is_own(&ctx.cache) {
            return;
        }

        let (chat_channel_id, rcon) = {
            let data_container = ctx.data.read();
            (
                data_container.get::<ChannelIdContainer>().unwrap().clone(),
                data_container.get::<RconContainer>().unwrap().clone(),
            )
        };

        if message.channel_id != chat_channel_id {
            return;
        }

        let nick = match message.author_nick(ctx.http) {
            Some(n) => n,
            None => message.author.name.clone(),
        };
        let content = message.content_safe(ctx.cache);
        println!("Bridge: D->A ({}): {}", nick, content);

        let mut rcon = rcon.lock().unwrap();
        if let Err(e) = rcon.exec(format!("ServerChat (D) {}: {}", nick, content).as_str()) {
            println!("RCON: Could not send message: {}", e.to_string());
        }
    }
}
