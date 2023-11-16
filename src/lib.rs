use core::GameState;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_rapier2d::prelude::*;
use map::ShipParameters;

pub mod ai;
pub mod camera;
pub mod core;
pub mod fov;
pub mod map;
pub mod player;
pub mod rand;
pub mod setup;
pub mod sprites;

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

    // Adding the plugin, even though we promptly replace the RNG, ensures our types are registered with Bevy
    app.add_plugins(rand::RandPlugin::default())
        // TODO: Allow world seed to be configurable
        .insert_resource(rand::world_seed("Test seed"));

    app.init_resource::<ShipParameters>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<core::GameState>()
        .add_systems(Startup, (camera::spawn_camera, sprites::load_sprites))
        .add_systems(
            Update,
            (
                core::advance_game_state,
                player::player_debug,
                fov::add_fov,
                fov::update_fov.after(fov::add_fov),
                fov::update_viewables,
                ai::drone_idle,
                (
                    player::player_walk,
                    player::player_face,
                    // map::debug_triangulation,
                )
                    .run_if(in_state(GameState::InGame)),
            ),
        )
        // Update camera position in PostUpdate, but before Bevy propagates Transform to GlobalTransform
        .add_systems(
            PostUpdate,
            camera::follow_entity.after(TransformSystem::TransformPropagate),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                // map::setup_map,
                setup::test_rng,
                player::spawn_player,
                fov::setup_fog_of_war,
                setup::setup_test_entities,
            ),
        )
        .run();
}
