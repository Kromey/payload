use std::f32::consts::TAU;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        mesh::Indices,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::{camera::Follow, core::OPAQUE_GROUP};

use super::FieldOfView;

/// How much to rotate each ray when calculating view cone
const RAY_ROTATION_ANGLE: f32 = TAU / 360.0;

enum FovLayer {
    Static,
    Dynamic,
}

impl FovLayer {
    fn z(&self) -> f32 {
        match self {
            FovLayer::Static => 199.0,
            FovLayer::Dynamic => 200.0,
        }
    }

    fn texture_render_layers(&self) -> RenderLayers {
        match self {
            FovLayer::Static => RenderLayers::default().with(3),
            FovLayer::Dynamic => RenderLayers::default(),
        }
    }

    fn camera_render_layers(&self) -> RenderLayers {
        match self {
            FovLayer::Static => RenderLayers::layer(1),
            FovLayer::Dynamic => RenderLayers::layer(2),
        }
    }
}

pub(super) fn make_fov_texture(fov: &FieldOfView) -> Image {
    let extent = fov.view_distance as u32 * 2;
    let size = Extent3d {
        width: extent,
        height: extent,
        ..Default::default()
    };

    // This is the texture that the field of view cone will be rendered to.
    // Need to do this manually to specify the usages
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    // Since we created this manually, we need to resize the data buffer
    image.resize(size);

    image
}

pub(super) fn make_fov_layers(
    viewer: Entity,
    image: Image,
    mesh: Handle<Mesh>,
    images: &mut Assets<Image>,
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
) {
    for layer in [FovLayer::Dynamic, FovLayer::Static] {
        let render_target = images.add(image.clone());
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: materials.add(ColorMaterial::from(render_target.clone())),
                transform: Transform::from_xyz(0.0, 0.0, layer.z()),
                ..Default::default()
            },
            layer.texture_render_layers(),
        ));
        commands.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(render_target),
                    ..Default::default()
                },
                ..Default::default()
            },
            Follow(viewer),
            layer.camera_render_layers(),
        ));
    }
}

pub(super) fn cast_view_cone(
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

pub(super) fn update_view_mesh(mesh: &mut Mesh, points: &[Vec2], fov: &FieldOfView, origin: Vec2) {
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
