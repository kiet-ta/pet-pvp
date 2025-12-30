use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_renet::RenetServerPlugin;
use bevy_renet::netcode::NetcodeServerPlugin;
use shared::{AppState, SharedPlugin};

mod network;
mod resources;
mod systems;

use network::{new_renet_server, process_packets, server_events, sync_players};
use resources::{LastInputTicks, Lobby};
use systems::auto_start_game;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin); // Transport Layer
    app.add_plugins(SharedPlugin);

    app.init_resource::<Lobby>();
    app.init_resource::<LastInputTicks>();

    // Server & Transport Setup
    let (server, transport) = new_renet_server();
    app.insert_resource(server);
    app.insert_resource(transport);

    app.add_systems(
        Update,
        (
            server_events,                                      // Handle Connect/Disconnect
            auto_start_game,                                    // Transition to InGame
            process_packets.run_if(in_state(AppState::InGame)), // Handle Inputs
            sync_players.run_if(in_state(AppState::InGame)),    // Send positions to clients
        ),
    );

    app.run();
}
