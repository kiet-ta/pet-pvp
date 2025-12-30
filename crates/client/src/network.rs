use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use shared::AppState;
use shared::movement::shared_movement_logic;
use shared::protocol::{
    ClientMessages, PROTOCOL_ID, PlayerInput, ServerMessages, UNRELIABLE_CHANNEL_ID,
};
use std::net::UdpSocket;
use std::time::SystemTime;

use crate::resources::{InputBuffer, Lobby};

pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64; // Simple ID generation

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

// 2. Send Input
pub fn send_input(mut client: ResMut<RenetClient>, input: Res<PlayerInput>) {
    if !client.is_connected() {
        return;
    }

    let message = ClientMessages::PlayerInput { input: *input };
    let serialized = bincode::serialize(&message).unwrap();
    client.send_message(UNRELIABLE_CHANNEL_ID, serialized);
}

// 3. Receive & Sync Visuals (Server Reconciliation)
#[allow(clippy::too_many_arguments)]
pub fn client_sync(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    transport: Res<NetcodeClientTransport>, // To get our ID
    mut query: Query<(&mut Transform, &mut Velocity)>,
    mut input_buffer: ResMut<InputBuffer>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    let client_id = transport.client_id();

    while let Some(message) = client.receive_message(UNRELIABLE_CHANNEL_ID) {
        if let Ok(msg) = bincode::deserialize::<ServerMessages>(&message) {
            match msg {
                ServerMessages::PlayerSync {
                    id,
                    position,
                    velocity: server_velocity,
                    last_input_tick,
                } => {
                    if let Some(&entity) = lobby.players.get(&id) {
                        // Check if this is US (Local Player)
                        if id == client_id {
                            // SERVER RECONCILIATION
                            if let Some(last_tick) = last_input_tick {
                                // 1. Remove inputs that have been confirmed by server
                                input_buffer.inputs.retain(|i| i.tick > last_tick);

                                // 2. Snap to Server State
                                if let Ok((mut transform, mut vel)) = query.get_mut(entity) {
                                    transform.translation.x = position.x;
                                    transform.translation.y = position.y;
                                    vel.linvel = server_velocity;

                                    // 3. Replay inputs (Re-simulate future)
                                    // Use a fixed delta time approximation.
                                    // ideally this should match FixedUpdate step exactly.
                                    let dt = 1.0 / 60.0;

                                    // We need to simulate the physics integration for the replay
                                    let mut temp_vel = server_velocity;
                                    let mut temp_pos = position;

                                    for input in input_buffer.inputs.iter() {
                                        // Apply Logic
                                        // Note: We need to reconstruct a Velocity struct or pass the values
                                        // shared_movement_logic expects &mut Velocity.
                                        // We'll create a dummy Velocity to simulate.
                                        let mut sim_vel = Velocity {
                                            linvel: temp_vel,
                                            angvel: 0.0,
                                        };

                                        shared_movement_logic(&mut sim_vel, input);

                                        temp_vel = sim_vel.linvel;
                                        // Manual Integration (Euler)
                                        temp_pos += temp_vel * dt;
                                    }

                                    // Apply the re-simulated state to the actual component
                                    // This "Predicts" where we should be now based on Server Fact + Local Input History
                                    transform.translation.x = temp_pos.x;
                                    transform.translation.y = temp_pos.y;
                                    vel.linvel = temp_vel;
                                }
                            }
                        } else {
                            // Other Players: Just Snap (Interpolation would go here)
                            if let Ok((mut transform, mut vel)) = query.get_mut(entity) {
                                transform.translation.x = position.x;
                                transform.translation.y = position.y;
                                vel.linvel = server_velocity;
                            }
                        }
                    } else {
                        // Spawn new player visual
                        println!("Spawning visual for Player {}", id);
                        let is_local = id == client_id;
                        let color = if is_local {
                            Color::srgb(0.0, 1.0, 0.0)
                        } else {
                            Color::srgb(1.0, 0.0, 0.0)
                        };

                        let entity = commands
                            .spawn((
                                Sprite {
                                    color,
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                Transform::from_xyz(position.x, position.y, 0.0),
                                Velocity {
                                    linvel: server_velocity,
                                    angvel: 0.0,
                                }, // Initialize with server velocity
                                RigidBody::Dynamic, // Client needs RB for prediction to work with Rapier?
                                shared::components::Player,
                            ))
                            .id();

                        lobby.players.insert(id, entity);

                        // Transition to InGame if we spawn a player (meaning we are connected and game is running)
                        if *state.get() == AppState::Lobby {
                            println!("Player spawned. Transitioning to InGame.");
                            next_state.set(AppState::InGame);
                        }
                    }
                }
                ServerMessages::PlayerDisconnected { id } => {
                    if let Some(entity) = lobby.players.remove(&id) {
                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}
