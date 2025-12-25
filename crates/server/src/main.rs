use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::renet::{
    ConnectionConfig, RenetServer, ServerEvent,
};
use bevy_renet::RenetServerPlugin;
// Adjusting imports based on common bevy_renet structure
use bevy_renet::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use shared::{PlayerInput, ServerMessages, SharedPlugin, PROTOCOL_ID};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

// Lobby: Map ClientId (u64) -> Entity (in Bevy World)
#[derive(Resource, Default)]
struct Lobby {
    players: HashMap<u64, Entity>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin); // Transport Layer
    app.add_plugins(SharedPlugin);

    app.init_resource::<Lobby>();
    
    // Server & Transport Setup
    let (server, transport) = new_renet_server();
    app.insert_resource(server);
    app.insert_resource(transport);

    app.add_systems(Update, (
        server_events,      // Handle Connect/Disconnect
        process_packets,    // Handle Inputs
        sync_players,       // Send positions to clients
    ));

    app.run();
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    
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
fn server_events(
    mut events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected.", client_id);
                
                // Spawn player entity (Authoritative)
                let player_entity = commands.spawn((
                    shared::Player, // Tag component
                    Transform::from_xyz(0.0, 100.0, 0.0),
                    RigidBody::Dynamic,
                    Collider::cuboid(25.0, 25.0),
                    Velocity::default(),
                    LockedAxes::ROTATION_LOCKED,
                )).id();

                // Map ID -> Entity
                lobby.players.insert(*client_id, player_entity);

                // Notify everyone (Optional, but good for reliable spawn)
                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(shared::RELIABLE_CHANNEL_ID, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {}", client_id, reason);
                if let Some(entity) = lobby.players.remove(client_id) {
                    commands.entity(entity).despawn();
                }
                
                let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
                server.broadcast_message(shared::RELIABLE_CHANNEL_ID, message);
            }
        }
    }
}

// 2. Process Inputs: Client -> Server
fn process_packets(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut query: Query<&mut Velocity>, 
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, shared::UNRELIABLE_CHANNEL_ID) {
            if let Ok(shared::ClientMessages::PlayerInput { action }) = bincode::deserialize(&message) {
                // Apply input to the SPECIFIC entity owned by this Client
                if let Some(&entity) = lobby.players.get(&client_id) {
                    if let Ok(mut velocity) = query.get_mut(entity) {
                        apply_input(&mut velocity, action);
                    }
                }
            }
        }
    }
}

fn apply_input(velocity: &mut Velocity, input: PlayerInput) {
    let speed = 200.0;
    velocity.linvel.x = input.move_axis * speed;
    if input.jump && velocity.linvel.y.abs() < 0.1 {
        velocity.linvel.y = 400.0;
    }
}

// 3. Sync State: Server -> Client
fn sync_players(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    query: Query<&Transform>,
) {
    for (client_id, &entity) in lobby.players.iter() {
        if let Ok(transform) = query.get(entity) {
            let message = ServerMessages::PlayerSync {
                id: *client_id,
                position: transform.translation.truncate(), // Vec3 -> Vec2
            };
            
            let serialized = bincode::serialize(&message).unwrap();
            // Broadcast position to ALL clients (so everyone sees everyone)
            server.broadcast_message(shared::UNRELIABLE_CHANNEL_ID, serialized);
        }
    }
}