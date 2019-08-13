extern crate envmnt;
extern crate rercon;
#[macro_use]
extern crate lazy_static;
extern crate serenity;

use std::{env, thread};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serenity::model::id::ChannelId;
use serenity::prelude::TypeMapKey;
use serenity::Client;

use rercon::ReConnection;
use start_error::StartError;

mod ark;
mod discord;
mod start_error;

fn main() -> Result<(), StartError> {
    if !envmnt::is_all_exists(&vec![
        "RCON_HOST",
        "RCON_PASS",
        "DISCORD_TOKEN",
        "DISCORD_CHANNEL",
    ]) {
        return Err(StartError::from("Bridge: Not all of the RCON_HOST, RCON_PASS, DISCORD_TOKEN and DISCORD_CHANNEL environment vars have been set. Exiting".to_string()));
    }

    let channel = ChannelId(env::var("DISCORD_CHANNEL")?.parse::<u64>()?);

    println!("Starting ARK<->Discord chat bridge...");
    println!("RCON: Connecting...");
    let rcon = Arc::new(Mutex::new(ReConnection::open(
        env::var("RCON_HOST")?,
        env::var("RCON_PASS")?,
        Some(Duration::from_secs(10)),
    )?));
    println!("RCON: Connected & reading!");

    println!("Discord: Setting up...");
    let mut discord = Client::new(env::var("DISCORD_TOKEN")?, discord::Handler)?;
    {
        let mut data = discord.data.write();
        data.insert::<RconContainer>(rcon.clone());
        data.insert::<ChannelIdContainer>(channel);
    }

    ark::start_loop(rcon, discord.cache_and_http.clone(), channel)?;

    thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Discord: Connected & waiting for events!");
    });

    if let Err(e) = discord.start_autosharded() {
        return Err(StartError::from(e));
    }

    Ok(())
}

struct RconContainer;

impl TypeMapKey for RconContainer {
    type Value = Arc<Mutex<ReConnection>>;
}

struct ChannelIdContainer;

impl TypeMapKey for ChannelIdContainer {
    type Value = ChannelId;
}
