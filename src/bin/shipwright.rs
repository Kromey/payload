use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(payload::rand::RandPlugin::default())
        .add_systems(
            Startup,
            (
                payload::camera::spawn_camera,
                payload::sprites::load_sprites,
                payload::map::setup_map,
            ),
        )
        .add_systems(Update, payload::map::debug_triangulation)
        .run();
}
