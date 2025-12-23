use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SharePlugin;
impl Plugin for SharePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    }
}
