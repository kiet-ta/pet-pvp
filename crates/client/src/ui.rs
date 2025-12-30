use bevy::prelude::*;
use shared::AppState;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn menu_logic(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Press Enter to Start (Go to Lobby/Connect)
    if *state.get() == AppState::Menu && keys.just_pressed(KeyCode::Enter) {
        println!("Starting Game: Menu -> Lobby");
        next_state.set(AppState::Lobby);
    }
}
