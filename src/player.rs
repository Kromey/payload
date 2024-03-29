use std::f32::consts::TAU;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::{Follow, MainCamera},
    core::PLAYER_GROUP,
    fov::FieldOfView,
    sprites::Sprites,
};

//FIXME: This should be a component on the player
const PLAYER_MOVE_SPEED: f32 = 150.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    camera_qry: Query<Entity, With<MainCamera>>,
) {
    let player_entity = commands
        .spawn((
            SpriteBundle {
                texture: sprites.player.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..Default::default()
            },
            Collider::capsule(Vec2::new(0.0, -5.0), Vec2::new(0.0, 5.0), 12.0),
            KinematicCharacterController {
                custom_mass: Some(50.0),
                ..Default::default()
            },
            Velocity::default(),
            Player,
            CollisionGroups::new(PLAYER_GROUP, Group::all()),
            FieldOfView::new(256.0, TAU / 12.0),
        ))
        .id();
    // Make sure the game's camera follows the player
    for camera in camera_qry.iter() {
        commands.entity(camera).insert(Follow(player_entity));
    }
}

pub fn player_debug(player_qry: Query<&GlobalTransform, With<Player>>, mut gizmos: Gizmos) {
    for player_transform in player_qry.iter() {
        let position = player_transform.translation().truncate();
        let looking = player_transform.right().truncate();

        gizmos.arc_2d(
            position,
            looking.angle_between(Vec2::Y),
            TAU / 3.0,
            20.0,
            Color::GREEN.with_a(0.5),
        );
        gizmos.ray_2d(
            position + looking * 18.0,
            looking * 10.0,
            Color::GREEN.with_a(0.5),
        );
    }
}

pub fn player_walk(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_qry: Query<&mut KinematicCharacterController, With<Player>>,
) {
    if let Ok(mut player_velocity) = player_qry.get_single_mut() {
        let mut velocity = Vec2::ZERO;

        if keys.pressed(KeyCode::W) {
            velocity += Vec2::Y;
        }
        if keys.pressed(KeyCode::A) {
            velocity += Vec2::NEG_X;
        }
        if keys.pressed(KeyCode::S) {
            velocity += Vec2::NEG_Y;
        }
        if keys.pressed(KeyCode::D) {
            velocity += Vec2::X;
        }

        velocity = velocity.normalize_or_zero() * PLAYER_MOVE_SPEED * time.delta_seconds();
        if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            velocity *= 0.2;
        }

        if velocity.length_squared() > f32::EPSILON {
            // Use get_or_insert and then add our velocity
            // This preserves any forces acting on the player added by other systems
            *player_velocity.translation.get_or_insert(Vec2::ZERO) += velocity;
        }
    }
}

pub fn player_face(
    mut player_qry: Query<&mut Transform, With<Player>>,
    window_qry: Query<&Window, With<PrimaryWindow>>,
    camera_qry: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Ok(window) = window_qry.get_single() {
        if let Some(cursor) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera_qry.get_single() {
                if let Some(look) = camera.viewport_to_world_2d(camera_transform, cursor) {
                    if let Ok(mut player_transform) = player_qry.get_single_mut() {
                        let to = look - player_transform.translation.truncate();
                        let facing = player_transform.local_x().truncate();
                        let angle = facing.angle_between(to);
                        if angle.abs() > f32::EPSILON {
                            player_transform.rotate_z(facing.angle_between(to));
                        }
                    }
                }
            }
        }
    }
}
