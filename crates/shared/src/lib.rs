pub mod components;
pub mod config;
pub mod movement;
pub mod protocol;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::setup_scene;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Lobby,    // Waiting for connection/players
    InGame,   // Active combat
    GameOver, // Finished
}

// =================================================================================
// PLUGINS & SYSTEMS
// Where to assemble features into the Bevy Engine.
// =================================================================================

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .init_state::<AppState>()
            .add_systems(Startup, setup_scene);
    }
}
