use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    PlayerInput { action: crate::PlayerAction },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessages {
    PlayerStateSync { translation: Vec2, entity_id: u32 },
}

// Config channel for renet (like choose channel radio)
pub const RELIABLE_CHANNEL_ID: u8 = 0;
pub const UNRELIABLE_CHANNEL_ID: u8 = 1; // use input/position (fastest)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    // Client send Input and Frame for Server
    InputUpdate {
        frame: u32,
        action: crate::PlayerAction,
    },
    // Server send state world (snapshot) for Client
    WorldStateSync {
        frame: u32,
        player_positions: Vec<(u32, bevy::prelude::Vec2)>, // ID and positions
    },
}
