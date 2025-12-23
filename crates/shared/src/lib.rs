use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

// --- MILESTONE 1: INPUT ---

// Define the actions that a Player can perform
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerInput {
    pub move_axis: f32, // Horizontal movement: -1.0 (Left) to 1.0 (Right)
    pub jump: bool,
}

// Resource to store Input of ALL players (Entity -> Input)
// The Server will receive this from the Network, the Client will populate this from the Keyboard
#[derive(Resource, Default)]
pub struct PlayerInputs {
    pub map: HashMap<Entity, PlayerInput>,
}

#[derive(Component)]
pub struct Player;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .init_resource::<PlayerInputs>()
            .add_systems(Startup, setup_scene)
            // Important: Movement logic must run in FixedUpdate for consistency
            .add_systems(FixedUpdate, player_movement_system);
    }
}

fn setup_scene(mut commands: Commands) {
    // Spawn Player
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
        Collider::cuboid(500.0, 25.0), // Half-extents
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));
}

fn player_movement_system(
    inputs: Res<PlayerInputs>,
    mut query: Query<(Entity, &mut Velocity), With<Player>>,
) {
    for (entity, mut vel) in query.iter_mut() {
        if let Some(input) = inputs.map.get(&entity) {
            let speed = 200.0;
            
            // Apply horizontal velocity
            vel.linvel.x = input.move_axis * speed;

            // Apply jump
            if input.jump {
                if vel.linvel.y.abs() < 0.1 {
                    // Only allow jumping when grounded
                    vel.linvel.y = 400.0;
                }
            }
        }
    }
}