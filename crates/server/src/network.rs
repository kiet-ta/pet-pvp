use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, RenetServer, ServerEvent};
use shared::movement::shared_movement_logic;
use shared::protocol::{
    ClientMessages, PROTOCOL_ID, RELIABLE_CHANNEL_ID, ServerMessages, UNRELIABLE_CHANNEL_ID,
};
use std::net::UdpSocket;
use std::time::SystemTime;

use crate::resources::{LastInputTicks, Lobby};

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}

// 1. Manage Connections: Spawn/Despawn Players
pub fn server_events(
    mut events: MessageReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    mut last_input_ticks: ResMut<LastInputTicks>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected.", client_id);

                // Spawn player entity (Authoritative)
                let player_entity = commands
                    .spawn((
                        shared::components::Player, // Tag component
                        Transform::from_xyz(0.0, 100.0, 0.0),
                        RigidBody::Dynamic,
                        Collider::cuboid(25.0, 25.0),
                        Velocity::default(),
                        LockedAxes::ROTATION_LOCKED,
                    ))
                    .id();

                // Map ID -> Entity
                lobby.players.insert(*client_id, player_entity);

                // Notify everyone (Optional, but good for reliable spawn)
                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(RELIABLE_CHANNEL_ID, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {}", client_id, reason);
                if let Some(entity) = lobby.players.remove(client_id) {
                    commands.entity(entity).despawn();
                }
                last_input_ticks.map.remove(client_id);

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(RELIABLE_CHANNEL_ID, message);
            }
        }
    }
}

// 2. Process Inputs: Client -> Server
#[allow(clippy::collapsible_if)]
pub fn process_packets(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut last_input_ticks: ResMut<LastInputTicks>,
    mut query: Query<&mut Velocity>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, UNRELIABLE_CHANNEL_ID) {
            if let Ok(ClientMessages::PlayerInput { input }) = bincode::deserialize(&message) {
                // Apply input to the SPECIFIC entity owned by this Client
                if let Some(&entity) = lobby.players.get(&client_id) {
                    if let Ok(mut velocity) = query.get_mut(entity) {
                        // Apply shared logic (DOD)
                        shared_movement_logic(&mut velocity, &input);

                        // Update the last processed tick for this client
                        last_input_ticks.map.insert(client_id, input.tick);
                    }
                }
            }
        }
    }
}

// 3. Sync State: Server -> Client
pub fn sync_players(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    last_input_ticks: Res<LastInputTicks>,
    query: Query<(&Transform, &Velocity)>,
) {
    for (client_id, &entity) in lobby.players.iter() {
        if let Ok((transform, velocity)) = query.get(entity) {
            let last_tick = last_input_ticks.map.get(client_id).copied();

            let message = ServerMessages::PlayerSync {
                id: *client_id,
                position: transform.translation.truncate(), // Vec3 -> Vec2
                velocity: velocity.linvel,
                last_input_tick: last_tick,
            };

            let serialized = bincode::serialize(&message).unwrap();
            // Broadcast position to ALL clients (so everyone sees everyone)
            server.broadcast_message(UNRELIABLE_CHANNEL_ID, serialized);
        }
    }
}
