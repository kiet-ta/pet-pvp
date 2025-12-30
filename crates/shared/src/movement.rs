use crate::config::{GROUND_CHECK_EPSILON, PLAYER_JUMP_FORCE, PLAYER_SPEED};
use crate::protocol::PlayerInput;
use bevy_rapier2d::prelude::*;

/// Shared movement logic for both Client (Prediction) and Server (Authority)
pub fn shared_movement_logic(velocity: &mut Velocity, input: &PlayerInput) {
    // Handle horizontal movement
    velocity.linvel.x = input.move_axis * PLAYER_SPEED;

    // Handle jump
    if input.jump {
        // Only allow jumping when Y velocity is near 0 (standing on ground)
        // Note: This logic is simple, effectively needs Raycast to check ground
        if velocity.linvel.y.abs() < GROUND_CHECK_EPSILON {
            velocity.linvel.y = PLAYER_JUMP_FORCE;
        }
    }
}
