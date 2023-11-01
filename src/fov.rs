use bevy::{prelude::*, render::mesh::Indices};
use bevy_rapier2d::prelude::*;
use itertools::Itertools;
use std::f32::consts::TAU;

use crate::{core::OPAQUE_GROUP, player::Player};

/// How much to rotate each ray when calculating view cone
const RAY_ROTATION_ANGLE: f32 = TAU / 360.0;

/// FIXME: These should be debug options that can be switched on/off at runtime
const DEBUG_RAYS: bool = false;
const DEBUG_VIEW_CONE: bool = true;
const DRAW_MAXIMAL_VIEW_CONE: bool = true;

#[derive(Debug, Clone, Component)]
pub struct FieldOfView {
    pub view_distance: u32,
    pub mesh: Handle<Mesh>,
    pub texture: Handle<Image>,
}

pub fn update_fov(
    rapier_context: Res<RapierContext>,
    player_qry: Query<(Entity, &GlobalTransform, &FieldOfView), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: Gizmos,
) {
    let maximal_color = Color::GRAY;
    let ray_color = Color::NAVY;
    let viewable_color = Color::PURPLE;

    let solid = true;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::all(), OPAQUE_GROUP));
    let view_cone = TAU / 12.0; // Vision only extends ±this angle
    let num_rays = (view_cone * 2.0 / RAY_ROTATION_ANGLE).floor() as usize;

    for (player, player_transform, player_fov) in player_qry.iter() {
        let max_toi = player_fov.view_distance as f32;
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
        if DEBUG_VIEW_CONE {
            // We need to add an additional origin point to the end to close the cone
            let mut positions = points.clone();
            positions.push(player_pos);
            gizmos.linestrip_2d(positions, viewable_color);
        }

        // Update our mesh
        let mesh = meshes.get_mut(&player_fov.mesh).unwrap();
        // Convert our Vec2 points into [f32; 3] arrays
        let mesh_points = points
            .iter()
            .map(|point| [point.x, point.y, 0.0])
            .collect_vec();
        // Calculate UV map where [0,0] is top left and [1,1] is bottom right
        // Note that UV coordinates are for the entire square that our view disc is inscribed within, not just the cone itself!
        let uv_origin = Vec2::new(player_pos.x - max_toi, player_pos.y - max_toi);
        let uv_points = points
            .iter()
            .map(|&point| {
                let uv = (point - uv_origin) / max_toi / 2.0;
                // Flip y: In Bevy space, y points up; in UV space, y points down!
                [uv.x, 1.0 - uv.y]
            })
            .collect_vec();
        // We need a list of triangles in order to generate our mesh
        // Skip 0 initially as we'll insert it for each triangle
        let indices = (1..mesh_points.len()).collect_vec();
        let triangles = indices
            // Use windows to get every (overlapping) pair of adjacent indices
            .windows(2)
            // Stick 0 in front of the pair, then flatten the whole thing
            .flat_map(|pair| [0, pair[0] as u32, pair[1] as u32])
            .collect_vec();

        // Now update our mesh with all this stuff!
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; points.len()]);
        mesh.set_indices(Some(Indices::U32(triangles)));
    }
}
