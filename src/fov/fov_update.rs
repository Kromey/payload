use std::f32::consts::TAU;

use bevy::{prelude::*, render::mesh::Indices};
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::core::OPAQUE_GROUP;

use super::FieldOfView;

/// How much to rotate each ray when calculating view cone
const RAY_ROTATION_ANGLE: f32 = TAU / 360.0;

pub fn cast_view_cone(
    viewer: Entity,
    origin: Vec2,
    fov: &FieldOfView,
    view_direction: Vec2,
    rapier_context: &RapierContext,
) -> Vec<Vec2> {
    let filter = QueryFilter::new()
        .groups(CollisionGroups::new(Group::all(), OPAQUE_GROUP))
        .exclude_collider(viewer);
    let solid = true;

    let mut ray = Vec2::from_angle(-fov.view_angle).rotate(view_direction);
    let num_rays = (fov.view_angle * 2.0 / RAY_ROTATION_ANGLE).floor() as usize;

    // Iterate inclusively to ensure we include both edges of our view cone
    // OPTIMIZATION: We could collect corners and only cast rays to those, and then calculate the intersections
    // of the arc with each line segment and cast rays to those, to cast a lot fewer rays
    let mut points = vec![origin];
    points.extend((0..=num_rays).map(|_| {
        let toi = rapier_context
            .cast_ray(origin, ray, fov.view_distance, solid, filter)
            .map(|(_, toi)| toi)
            .unwrap_or(fov.view_distance);
        let end = origin + ray * toi;
        ray = ray.rotate(Vec2::from_angle(RAY_ROTATION_ANGLE));

        end
    }));

    points
}

pub fn update_view_mesh(mesh: &mut Mesh, points: &[Vec2], fov: &FieldOfView, origin: Vec2) {
    // Note that UV coordinates are for the entire square that our view disc is inscribed within, not just the cone itself!
    let uv_origin = Vec2::new(origin.x - fov.view_distance, origin.y - fov.view_distance);
    let (mesh_points, uv_points): (Vec<_>, Vec<_>) = points
        .iter()
        .map(|&point| {
            let uv = (point - uv_origin) / (fov.view_distance * 2.0);

            (
                [point.x, point.y, 0.0],
                // Flip y: In Bevy space, y points up; in UV space, y points down!
                [uv.x, 1.0 - uv.y],
            )
        })
        .unzip();
    // We need a list of triangles in order to generate our mesh
    // Generate this dynamically as we may dynamically alter the number of points
    let triangles = (1..mesh_points.len())
        .collect_vec()
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
