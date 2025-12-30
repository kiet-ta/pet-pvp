use crate::resources::Lobby;
use bevy::prelude::*;
use shared::AppState;

pub fn auto_start_game(
    lobby: Res<Lobby>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Simple logic: If we are in Lobby and have players, start game.
    // In real game, this might be a manual start command or timer.
    if *state.get() == AppState::Menu {
        // Server skips Menu, goes to Lobby
        next_state.set(AppState::Lobby);
    }

    if *state.get() == AppState::Lobby && !lobby.players.is_empty() {
        println!("Players connected. Starting Game...");
        next_state.set(AppState::InGame);
    }
}
