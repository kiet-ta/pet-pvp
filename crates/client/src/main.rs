use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use shared::SharedPlugin;
use shared::protocol::{
    ClientMessages, PROTOCOL_ID, PlayerInput, ServerMessages, UNRELIABLE_CHANNEL_ID,
};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

// Lobby to track Visual Entities on Client
#[derive(Resource, Default)]
struct Lobby {
    players: HashMap<u64, Entity>, // ClientId -> Entity (Sprite)
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin); // Transport Layer
    app.add_plugins(SharedPlugin);

    app.init_resource::<Lobby>();
    app.init_resource::<PlayerInput>(); // Store local input

    // Client & Transport Setup
    let (client, transport) = new_renet_client();
    app.insert_resource(client);
    app.insert_resource(transport);

    app.add_systems(Startup, setup_camera);
    // Correctly chaining systems
    app.add_systems(Update, (handle_input, send_input, client_sync));

    app.run();
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// 1. Collect Input
fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    let mut move_axis = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        move_axis -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_axis += 1.0;
    }

    // Use *input or input.move_axis depending on Deref behavior.
    // Since we are mutating, we modify the fields directly.
    input.move_axis = move_axis;
    input.jump = keys.pressed(KeyCode::Space);
}

// 2. Send Input
fn send_input(mut client: ResMut<RenetClient>, input: Res<PlayerInput>) {
    if !client.is_connected() {
        return;
    }

    let message = ClientMessages::PlayerInput { input: *input };
    let serialized = bincode::serialize(&message).unwrap();
    client.send_message(UNRELIABLE_CHANNEL_ID, serialized);
}

// 3. Receive & Sync Visuals
fn client_sync(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut transform_query: Query<&mut Transform>,
) {
    while let Some(message) = client.receive_message(UNRELIABLE_CHANNEL_ID) {
        if let Ok(msg) = bincode::deserialize::<ServerMessages>(&message) {
            match msg {
                ServerMessages::PlayerSync { id, position } => {
                    if let Some(&entity) = lobby.players.get(&id) {
                        // Update existing player position
                        if let Ok(mut transform) = transform_query.get_mut(entity) {
                            transform.translation.x = position.x;
                            transform.translation.y = position.y;
                        }
                    } else {
                        // Spawn new player visual
                        println!("Spawning visual for Player {}", id);
                        let entity = commands
                            .spawn((
                                Sprite {
                                    color: Color::srgb(0.0, 1.0, 0.0), // Client sees Green players
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                Transform::from_xyz(position.x, position.y, 0.0),
                            ))
                            .id();

                        lobby.players.insert(id, entity);
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
