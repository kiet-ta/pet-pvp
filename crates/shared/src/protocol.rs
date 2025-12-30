use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// =================================================================================
// NETWORK CONSTANTS
// Define channels and protocol ID so Client/Server establish identical connection.
// =================================================================================

pub const RELIABLE_CHANNEL_ID: u8 = 0;
pub const UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const PROTOCOL_ID: u64 = 7;

// =================================================================================
// DATA TRANSFER OBJECTS
// These structs contain pure data, no logic.
// =================================================================================

/// Player input data.
/// This struct is sent over the network, so it needs Serialize/Deserialize.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, Resource)]
pub struct PlayerInput {
    pub tick: u64,      // The client's simulation frame for this input
    pub move_axis: f32, // Horizontal movement: -1.0 (Left) to 1.0 (Right)
    pub jump: bool,     // Jump state
}

// =================================================================================
// NETWORK MESSAGES
// Define communication "Language".
// =================================================================================

/// Messages sent from Client -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessages {
    PlayerInput { input: PlayerInput },
}

/// Messages sent from Server -> Client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessages {
    PlayerConnected {
        id: u64,
    },
    PlayerDisconnected {
        id: u64,
    },
    // Position synchronization (snapshot)
    PlayerSync {
        id: u64,
        position: Vec2,
        velocity: Vec2, // Sync velocity too for smoother interpolation/prediction
        last_input_tick: Option<u64>, // The last input tick the server processed for this player
    },
}
