use bevy::prelude::*;
use shared::protocol::PlayerInput;
use std::collections::{HashMap, VecDeque};

// Lobby to track Visual Entities on Client
#[derive(Resource, Default)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>, // ClientId -> Entity (Sprite)
}

#[derive(Resource, Default)]
pub struct ClientTick {
    pub tick: u64,
}

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub inputs: VecDeque<PlayerInput>,
}
