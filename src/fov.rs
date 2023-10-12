use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{core::OPAQUE_GROUP, player::Player};

pub fn calculate_fov(
    rapier_context: Res<RapierContext>,
    player_qry: Query<(Entity, &GlobalTransform), With<Player>>,
    mut gizmos: Gizmos,
) {
    //FIXME: These points will exist in the map and should be queried from there
    let x1 = -32.0;
    let x2 = 32.0;
    let y1 = 128.0 - 16.0;
    let y2 = 128.0 + 16.0;

    let points = [
        Vec2::new(x1, y1),
        Vec2::new(x1, y2),
        Vec2::new(x2, y1),
        Vec2::new(x2, y2),
    ];

    let max_toi = 256.0;
    let solid = true;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::all(), OPAQUE_GROUP));
    let view_cone = TAU / 12.0; // Vision only extends ±this angle

    for (player, player_transform) in player_qry.iter() {
        let player_facing = player_transform.right().truncate();
        let player_pos = player_transform.translation().truncate();
        let filter = filter.exclude_collider(player);

        for &corner in points.iter() {
            let ray_dir = (corner - player_pos).normalize();
            if player_facing.angle_between(ray_dir).abs() <= view_cone {
                gizmos.ray_2d(
                    player_pos,
                    (corner - player_pos).clamp_length_max(max_toi),
                    Color::WHITE,
                );
                let toi = rapier_context
                    .cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                    .map(|(_, toi)| toi)
                    .unwrap_or(max_toi);
                gizmos.ray_gradient_2d(player_pos, ray_dir * toi, Color::GREEN, Color::RED);
            }
        }

        for angle in [Vec2::from_angle(view_cone), Vec2::from_angle(-view_cone)] {
            let ray_dir = player_facing.rotate(angle);
            gizmos.ray_2d(player_pos, ray_dir * max_toi, Color::WHITE);
            let toi = rapier_context
                .cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                .map(|(_, toi)| toi)
                .unwrap_or(max_toi);
            gizmos.ray_gradient_2d(player_pos, ray_dir * toi, Color::GREEN, Color::RED);
        }
    }
}
