use bevy::prelude::*;
use bevy::log::LogPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .run();
}