use core::GameState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod core;
mod player;
mod sprites;

pub fn run_game() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<()>::pixels_per_meter(32.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO, // In a top-down view, Rapier doesn't "see" gravity
            ..Default::default()
        });

    // Only enable Rapier debug rendering in debug builds
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Update, bevy::window::close_on_esc)
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
