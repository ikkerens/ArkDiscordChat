use std::sync::{Arc, Mutex};

use rercon::ReConnection;
use serenity::{model::id::ChannelId,
               prelude::TypeMapKey};

pub(crate) struct RconContainer;

impl TypeMapKey for RconContainer {
    type Value = Arc<Mutex<ReConnection>>;
}

pub(crate) struct ChannelIdContainer;

impl TypeMapKey for ChannelIdContainer {
    type Value = ChannelId;
}
