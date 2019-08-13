extern crate regex;

use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use regex::Regex;
use rercon::ReConnection;
use serenity::model::id::ChannelId;
use serenity::CacheAndHttp;
use rercon::Error::BusyReconnecting;

pub(crate) fn start_loop(
    rcon: Arc<Mutex<ReConnection>>,
    discord: Arc<CacheAndHttp>,
    channel: ChannelId,
) -> Result<()> {
    thread::Builder::new()
        .name("Ark-Loop".to_string())
        .spawn(move || ark_loop(rcon, discord, channel))
        .map(|_| ())
}

fn ark_loop(rcon: Arc<Mutex<ReConnection>>, discord: Arc<CacheAndHttp>, channel: ChannelId) {
    loop {
        {
            let mut lock = rcon.lock().unwrap();
            let log = match lock.exec("GetGameLog") {
                Err(e) => {
                    println!("RCON: Could not get game log: {}", e.to_string());
                    if let BusyReconnecting(_) = e {
                        thread::sleep(Duration::from_secs(1));
                    }
                    continue;
                }
                Ok(l) => l,
            };

            log.lines().for_each(|l| handle_line(&discord, &channel, l))
        }

        thread::sleep(Duration::from_millis(250))
    }
}

fn handle_line(discord: &Arc<CacheAndHttp>, channel: &ChannelId, line: &str) {
    lazy_static! {
        static ref CHAT_MESSAGE_PATTERN: Regex =
            Regex::new("(?:[0-9._]+): (?:[A-z0-9 ]+) \\(([A-z0-9 ]+?)\\): (.*)").unwrap();
        static ref JOIN_MESSAGE_PATTERN: Regex =
            Regex::new("(?:[0-9._]+): ([A-z ]+) (joined|left) this ARK!").unwrap();
        static ref SERVER_MESSAGE_PATTERN: Regex =
            Regex::new("(?:[0-9._]+): SERVER: (?:.*)").unwrap();
    }

    let mirror_msg = if let Some(groups) = CHAT_MESSAGE_PATTERN.captures(line) {
        let (name, message) = (
            groups.get(1).unwrap().as_str(),
            groups.get(2).unwrap().as_str(),
        );
        println!("Bridge: A->D ({}): {}", name, message);
        Some(format!("<**{}**> {}", name, message))
    } else if let Some(groups) = JOIN_MESSAGE_PATTERN.captures(line) {
        let (name, action) = (
            groups.get(1).unwrap().as_str(),
            groups.get(2).unwrap().as_str(),
        );
        println!("Bridge: A->D: {} {} the ARK!", name, action);
        Some(format!("***{}** {} the ARK!*", name, action))
    } else if SERVER_MESSAGE_PATTERN.is_match(line) {
        None
    } else {
        None
    };

    if let Some(msg) = mirror_msg {
        if let Err(e) = channel.send_message(&discord.http, move |m| m.content(msg)) {
            println!("Discord: Could not send message: {}", e.to_string());
        }
    }
}
