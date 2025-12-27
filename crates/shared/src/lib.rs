pub mod protocol;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

// Import "Communication Language" from protocol.rs
use crate::protocol::PlayerInput;

// =================================================================================
// COMPONENTS
// Components attached to Entities to define Game Logic.
// =================================================================================

#[derive(Component)]
pub struct Player;

// =================================================================================
// RESOURCES
// Global data used to manage logic.
// =================================================================================

/// Resource storing Input of ALL players (Entity -> Input).
/// The Server will update this from the network (ClientMessages).
/// The Client will update this from the keyboard (Local Input).
#[derive(Resource, Default)]
pub struct PlayerInputs {
    pub map: HashMap<Entity, PlayerInput>,
}

// =================================================================================
// PLUGINS & SYSTEMS
// Where to assemble features into the Bevy Engine.
// =================================================================================

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .init_resource::<PlayerInputs>()
            .add_systems(Startup, setup_scene)
            // Important: Physics movement logic should run in FixedUpdate for synchronization
            .add_systems(FixedUpdate, player_movement_system);
    }
}

/// Initialize game environment (Ground, Test Character...)
fn setup_scene(mut commands: Commands) {
    // Spawn Player (Temporarily spawn here to test physics logic)
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(25.0, 25.0),
        Velocity::default(),
        Transform::from_xyz(0.0, 200.0, 0.0),
        // Lock rotation so the character doesn't roll
        LockedAxes::ROTATION_LOCKED,
    ));

    // Spawn Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(1000.0, 50.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(500.0, 25.0), // Half-extents (equal to 1/2 actual size)
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));
}

/// System handling movement based on received Input
fn player_movement_system(
    inputs: Res<PlayerInputs>,
    mut query: Query<(Entity, &mut Velocity), With<Player>>,
) {
    for (entity, mut vel) in query.iter_mut() {
        if let Some(input) = inputs.map.get(&entity) {
            let speed = 200.0;

            // Handle horizontal movement
            vel.linvel.x = input.move_axis * speed;

            // Handle jump
            if input.jump {
                // Only allow jumping when Y velocity is near 0 (standing on ground)
                // Note: This logic is simple, effectively needs Raycast to check ground
                if vel.linvel.y.abs() < 0.1 {
                    vel.linvel.y = 400.0;
                }
            }
        }
    }
}
