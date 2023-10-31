use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{core::OPAQUE_GROUP, player::Player};

/// How much to rotate each ray when calculating view cone
const RAY_ROTATION_ANGLE: f32 = TAU / 360.0;

/// FIXME: These should be debug options that can be switched on/off at runtime
const DEBUG_RAYS: bool = false;
const DEBUG_VIEW_CONE: bool = true;
const DRAW_MAXIMAL_VIEW_CONE: bool = true;

pub fn calculate_fov(
    rapier_context: Res<RapierContext>,
    player_qry: Query<(Entity, &GlobalTransform), With<Player>>,
    mut gizmos: Gizmos,
) {
    let maximal_color = Color::GRAY;
    let ray_color = Color::NAVY;
    let viewable_color = Color::PURPLE;

    let max_toi = 256.0;
    let solid = true;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::all(), OPAQUE_GROUP));
    let view_cone = TAU / 12.0; // Vision only extends ±this angle
    let num_rays = (view_cone * 2.0 / RAY_ROTATION_ANGLE).floor() as usize;

    for (player, player_transform) in player_qry.iter() {
        let player_facing = player_transform.right().truncate();
        let player_pos = player_transform.translation().truncate();
        let filter = filter.exclude_collider(player);

        let mut ray = Vec2::from_angle(-view_cone).rotate(player_facing);

        if DRAW_MAXIMAL_VIEW_CONE {
            // ## Draw view cone outline ##
            gizmos.arc_2d(
                player_pos,
                player_facing.angle_between(Vec2::Y),
                view_cone * 2.0,
                max_toi,
                maximal_color,
            );
            for angle in [Vec2::from_angle(view_cone), Vec2::from_angle(-view_cone)] {
                let ray_dir = player_facing.rotate(angle);
                gizmos.ray_2d(player_pos, ray_dir * max_toi, maximal_color);
                // This is the edge of the view cone, always draw this ray even if it doesn't collide with anything
                let toi = rapier_context
                    .cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                    .map(|(_, toi)| toi)
                    .unwrap_or(max_toi);
                gizmos.ray_2d(player_pos, ray_dir * toi, maximal_color);
            }
        }

        // Iterate inclusively to ensure we draw both edges of our view cone
        // OPTIMIZATION: We could collect corners and only cast rays to those, and then calculate the intersections
        // of the arc with each line segment and cast rays to those, to cast a lot fewer rays
        let mut points = vec![player_pos];
        points.extend((0..=num_rays).map(|_| {
            let toi = rapier_context
                .cast_ray(player_pos, ray, max_toi, solid, filter)
                .map(|(_, toi)| toi)
                .unwrap_or(max_toi);
            let end = player_pos + ray * toi;
            if DEBUG_RAYS {
                gizmos.line_2d(player_pos, end, ray_color);
            }
            ray = ray.rotate(Vec2::from_angle(RAY_ROTATION_ANGLE));

            end
        }));
        points.push(player_pos);
        if DEBUG_VIEW_CONE {
            gizmos.linestrip_2d(points, viewable_color);
        }
    }
}
