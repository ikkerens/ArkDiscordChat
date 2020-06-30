extern crate regex;

use std::{sync::Arc, time::Duration};

use lazy_static::lazy_static;
use regex::Regex;
use rercon::{Error::BusyReconnecting, ReConnection};
use serenity::{model::id::ChannelId, CacheAndHttp};
use tokio::{sync::Mutex, time::delay_for};

pub(crate) fn start_loop(rcon: Arc<Mutex<ReConnection>>, discord: Arc<CacheAndHttp>, channel: ChannelId) {
	tokio::spawn(async move { ark_loop(rcon, discord, channel).await });
}

async fn ark_loop(rcon: Arc<Mutex<ReConnection>>, discord: Arc<CacheAndHttp>, channel: ChannelId) {
	loop {
		{
			let response = {
				let mut lock = rcon.lock().await;
				lock.exec("GetGameLog").await
			};

			let log = match response {
				Err(e) => {
					println!("RCON: Could not get game log: {}", e.to_string());
					if let BusyReconnecting(_) = e {
						delay_for(Duration::from_secs(1)).await;
					}
					continue;
				}
				Ok(l) => l,
			};

			for line in log.lines() {
				handle_line(&discord, channel, line).await
			}
		}

		delay_for(Duration::from_millis(250)).await
	}
}

async fn handle_line(discord: &Arc<CacheAndHttp>, channel: ChannelId, line: &str) {
	lazy_static! {
		static ref CHAT_MESSAGE_PATTERN: Regex =
			Regex::new("(?:[0-9._]+): (?:[A-z0-9 ]+) \\(([A-z0-9 ]+?)\\): (.*)").unwrap();
		static ref JOIN_MESSAGE_PATTERN: Regex = Regex::new("(?:[0-9._]+): ([A-z ]+) (joined|left) this ARK!").unwrap();
	}

	let mirror_msg = if let Some(groups) = CHAT_MESSAGE_PATTERN.captures(line) {
		let (name, message) = (groups.get(1).unwrap().as_str(), groups.get(2).unwrap().as_str());
		println!("Bridge: A->D ({}): {}", name, message);
		Some(format!("<**{}**> {}", name, message))
	} else if let Some(groups) = JOIN_MESSAGE_PATTERN.captures(line) {
		let (name, action) = (groups.get(1).unwrap().as_str(), groups.get(2).unwrap().as_str());
		println!("Bridge: A->D: {} {} the ARK!", name, action);
		Some(format!("***{}** {} the ARK!*", name, action))
	} else {
		None
	};

	if let Some(msg) = mirror_msg {
		if let Err(e) = channel.send_message(&discord.http, move |m| m.content(msg)).await {
			println!("Discord: Could not send message: {}", e.to_string());
		}
	}
}
