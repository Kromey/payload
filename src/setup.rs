use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ai::DroneAI,
    core::{OPAQUE_GROUP, PLAYER_GROUP},
    fov::{FieldOfView, Viewable},
    rand::*,
};

pub fn test_rng(mut world_rng: ResMut<WorldRng>) {
    let mut rng = world_rng.fork_inner();
    info!("Random number: {}", rng.gen::<u8>());
}

pub(crate) fn setup_test_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a drone that will share FoV with the player
    let drone_transform = Transform::from_xyz(179.0, 128.0, 5.0);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("drone.png"),
            transform: drone_transform.with_rotation(Quat::from_rotation_z(TAU / 1.8)),
            ..Default::default()
        },
        CollisionGroups::new(PLAYER_GROUP, Group::all()),
        FieldOfView::new(128.0, TAU / 10.0),
        DroneAI,
    ));

    // Spawn a collider so we can see how/if physics works
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 128.0, 1.0),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(64.0, 32.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(32.0, 16.0),
        CollisionGroups::new(OPAQUE_GROUP, Group::all()),
    ));

    // Spawn a few sprites so we can test field of view
    for (x, y) in [(128.0, 96.0), (96.0, 128.0), (-128.0, -32.0), (32.0, 0.0)] {
        let transform = Transform::from_xyz(x, y, 0.0);
        commands.spawn((
            SpriteBundle {
                transform,
                texture: asset_server.load("bevy_icon_32.png"),
                ..Default::default()
            },
            Viewable::Dynamic,
        ));
    }
}
