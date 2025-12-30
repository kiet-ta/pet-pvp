use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::netcode::NetcodeClientPlugin;
use shared::protocol::PlayerInput;
use shared::{AppState, SharedPlugin};

mod input;
mod network;
mod resources;
mod ui;

use input::handle_input;
use network::{client_sync, new_renet_client, send_input};
use resources::{ClientTick, InputBuffer, Lobby};
use ui::{menu_logic, setup_camera};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin); // Transport Layer
    app.add_plugins(SharedPlugin);

    app.init_resource::<Lobby>();
    app.init_resource::<ClientTick>();
    app.init_resource::<InputBuffer>();
    app.init_resource::<PlayerInput>(); // Store local input state

    // Client & Transport Setup
    let (client, transport) = new_renet_client();
    app.insert_resource(client);
    app.insert_resource(transport);

    app.add_systems(Startup, setup_camera);

    app.add_systems(Update, (menu_logic, client_sync));

    // Run Input & Prediction in FixedUpdate to match Physics/Server Tick
    app.add_systems(
        FixedUpdate,
        (
            handle_input.run_if(in_state(AppState::InGame)),
            send_input.run_if(in_state(AppState::InGame)),
        ),
    );

    app.run();
}
