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

    let maximal_color = Color::GRAY;
    let viewable_color = Color::NAVY;

    let max_toi = 256.0;
    let solid = true;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::all(), OPAQUE_GROUP));
    let view_cone = TAU / 12.0; // Vision only extends ±this angle
    let auxiliary_ray_angle = 0.001; // How wide to cast auxiliary rays when testing view past a corner

    for (player, player_transform) in player_qry.iter() {
        let player_facing = player_transform.right().truncate();
        let player_pos = player_transform.translation().truncate();
        let filter = filter.exclude_collider(player);

        gizmos.arc_2d(
            player_pos,
            player_facing.angle_between(Vec2::Y),
            view_cone * 2.0,
            max_toi,
            maximal_color,
        );

        for &corner in points.iter() {
            let ray_dir = (corner - player_pos).normalize();
            if player_facing.angle_between(ray_dir).abs() <= view_cone {
                gizmos.line_2d(player_pos, corner, maximal_color);
                if let Some((_, toi)) =
                    rapier_context.cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                {
                    let contact = player_pos + ray_dir * toi;
                    gizmos.line_2d(player_pos, contact, viewable_color);

                    // If we reached the corner, cast auxiliary rays
                    // See Section 2.2 here https://legends2k.github.io/2d-fov/design.html for an optimization,
                    // but it requires knowing the line segments, not just the list of corners
                    if contact.distance_squared(corner) < 1.0 {
                        gizmos.circle_2d(contact, 1.0, Color::RED);

                        for angle in [
                            Vec2::from_angle(auxiliary_ray_angle),
                            Vec2::from_angle(-auxiliary_ray_angle),
                        ] {
                            let ray_dir = ray_dir.rotate(angle);
                            if rapier_context
                                .cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                                .is_none()
                            {
                                let contact = player_pos + ray_dir * max_toi;
                                gizmos.line_2d(player_pos, contact, viewable_color);

                                // We've seen past this corner already, no reason to try the other side
                                break;
                            }
                        }
                    }
                }
            }
        }

        for angle in [Vec2::from_angle(view_cone), Vec2::from_angle(-view_cone)] {
            let ray_dir = player_facing.rotate(angle);
            gizmos.ray_2d(player_pos, ray_dir * max_toi, maximal_color);
            // This is the edge of the view cone, always draw this ray even if it doesn't collide with anything
            let toi = rapier_context
                .cast_ray(player_pos, ray_dir, max_toi, solid, filter)
                .map(|(_, toi)| toi)
                .unwrap_or(max_toi);
            gizmos.ray_2d(player_pos, ray_dir * toi, viewable_color);
        }
    }
}
