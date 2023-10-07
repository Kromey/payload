use core::GameState;

use bevy::prelude::*;

mod camera;
mod core;
mod player;
mod sprites;

pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<core::GameState>()
        .add_systems(Startup, (camera::spawn_camera, sprites::load_sprites))
        .add_systems(
            Update,
            (
                core::advance_game_state,
                player::player_debug,
                (player::player_walk, player::player_face).run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(OnEnter(GameState::InGame), player::spawn_player)
        .run();
}
