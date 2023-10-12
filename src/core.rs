use bevy::prelude::*;

mod physics_groups;
pub use physics_groups::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

/// FIXME: This only exists during development to advance unused states
pub fn advance_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match **state {
        GameState::Loading => next_state.set(GameState::MainMenu),
        GameState::MainMenu => next_state.set(GameState::InGame),
        GameState::InGame => {}
    }
}
