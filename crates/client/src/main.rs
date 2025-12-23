use bevy::prelude::*;
use shared::{Player, PlayerInput, PlayerInputs, SharedPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SharedPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_keyboard_input)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut inputs: ResMut<PlayerInputs>,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        let mut move_axis = 0.0;
        
        if keys.pressed(KeyCode::KeyA) {
            move_axis -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            move_axis += 1.0;
        }

        let jump = keys.pressed(KeyCode::Space);

        // Update the shared resource
        inputs.map.insert(entity, PlayerInput { move_axis, jump });
    }
}