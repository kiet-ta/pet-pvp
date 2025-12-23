use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::SharedPlugin;

fn main() {
    App::new()
        // MinimalPlugins make server light as possible
        .add_plugins(MinimalPlugins)
        .add_plugins(SharedPlugin)
        .add_systems(Update, server_log)
        .run();
}

fn server_log(query: Query<&Transform, With<RigidBody>>) {
    // print position Y and X of all rigid bodies
    for transform in &query {
        println!("Server: {}", transform.translation);
    }
}
