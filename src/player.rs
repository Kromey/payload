use std::f32::consts::TAU;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::MainCamera, sprites::Sprites};

//FIXME: This should be a component on the player
const PLAYER_MOVE_SPEED: f32 = 150.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn((
        SpriteBundle {
            texture: sprites.player.clone(),
            ..Default::default()
        },
        Player,
    ));
}

pub fn player_debug(player_qry: Query<&GlobalTransform, With<Player>>, mut gizmos: Gizmos) {
    for player_transform in player_qry.iter() {
        let position = player_transform.translation().truncate();
        let looking = player_transform.right().truncate();

        gizmos.circle_2d(position, 16.0, Color::BLUE.with_a(0.5));
        gizmos.arc_2d(
            position,
            looking.angle_between(Vec2::Y),
            TAU / 3.0,
            16.0,
            Color::GREEN.with_a(0.5),
        );
        gizmos.ray_2d(
            position + looking * 14.0,
            looking * 10.0,
            Color::GREEN.with_a(0.5),
        );
    }
}

pub fn player_walk(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_qry: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player) = player_qry.get_single_mut() {
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
        player.translation += velocity.extend(0.0);
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
                        player_transform.rotate_z(facing.angle_between(to));
                    }
                }
            }
        }
    }
}
