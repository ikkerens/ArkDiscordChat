use std::sync::Arc;

use rercon::ReConnection;
use serenity::{model::id::ChannelId, utils::TypeMapKey};
use tokio::sync::Mutex;

pub(crate) struct RconContainer;

impl TypeMapKey for RconContainer {
	type Value = Arc<Mutex<ReConnection>>;
}

pub(crate) struct ChannelIdContainer;

impl TypeMapKey for ChannelIdContainer {
	type Value = ChannelId;
}
