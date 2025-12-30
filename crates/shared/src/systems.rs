use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Initialize game environment (Ground)
pub fn setup_scene(mut commands: Commands) {
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
