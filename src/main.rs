extern crate envmnt;
#[macro_use]
extern crate lazy_static;
extern crate rercon;
extern crate serenity;

use std::{{env, thread},
          sync::{Arc, Mutex},
          time::Duration};

use rercon::ReConnection;
use serenity::{Client,
               model::id::ChannelId};
use serenity::framework::StandardFramework;

use start_error::StartError;

use crate::type_container::{ChannelIdContainer, RconContainer};

mod ark;
mod discord;
mod start_error;
mod type_container;
mod rcon_command;

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
    discord.with_framework(StandardFramework::new().configure(|c|
        c.prefix("!")
            .allow_dm(false)
    ).group(&rcon_command::GENERAL_GROUP));
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
