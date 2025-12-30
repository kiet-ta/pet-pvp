use crate::resources::{ClientTick, InputBuffer, Lobby};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_renet::netcode::NetcodeClientTransport;
use shared::components::Player;
use shared::movement::shared_movement_logic;
use shared::protocol::PlayerInput;

// 1. Collect Input & Predict (CSP)
#[allow(clippy::collapsible_if)]
pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
    mut tick: ResMut<ClientTick>,
    mut buffer: ResMut<InputBuffer>,
    mut query: Query<&mut Velocity, With<Player>>,
    lobby: Res<Lobby>,
    transport: Res<NetcodeClientTransport>,
) {
    let client_id = transport.client_id();

    // Increment Tick
    tick.tick += 1;

    let mut move_axis = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        move_axis -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_axis += 1.0;
    }

    input.move_axis = move_axis;
    input.jump = keys.pressed(KeyCode::Space);
    input.tick = tick.tick;

    // Save to buffer
    buffer.inputs.push_back(*input);

    // PREDICTION: Apply immediately
    if let Some(&entity) = lobby.players.get(&client_id) {
        if let Ok(mut velocity) = query.get_mut(entity) {
            shared_movement_logic(&mut velocity, &input);
        }
    }
}
