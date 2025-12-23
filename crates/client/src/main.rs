use bevy::prelude::*;
use shared::SharePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SharePlugin)
        .run();    
}
