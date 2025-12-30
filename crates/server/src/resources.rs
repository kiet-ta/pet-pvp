use bevy::prelude::*;
use std::collections::HashMap;

// Lobby: Map ClientId (u64) -> Entity (in Bevy World)
#[derive(Resource, Default)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}

// Stores the last input tick processed for each client
#[derive(Resource, Default)]
pub struct LastInputTicks {
    pub map: HashMap<u64, u64>,
}
