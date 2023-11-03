use std::f32::consts::TAU;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, PrimitiveTopology, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::{FollowPlayer, MainCamera},
    core::{OPAQUE_GROUP, PLAYER_GROUP},
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..Default::default()
    };
    // This is the texture that the player's view cone will be rendered to.
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
    image.resize(size);

    // This will be the mesh for the player's field of view
    let mesh_handle = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
    let render_target = images.add(image.clone());
    let entity_render_target = images.add(image.clone());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: materials.add(ColorMaterial::from(render_target.clone())),
            transform: Transform::from_xyz(0.0, 0.0, 199.0),
            ..Default::default()
        },
        RenderLayers::default().with(3),
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.clone().into(),
        material: materials.add(ColorMaterial::from(entity_render_target.clone())),
        transform: Transform::from_xyz(0.0, 0.0, 200.0),
        ..Default::default()
    });
    commands.spawn((
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
        FieldOfView {
            view_distance: 256.0,
            view_angle: TAU / 12.0,
            mesh: mesh_handle,
            texture: render_target.clone(),
        },
    ));
    // Spawn a camera that will be used to render the player's field of view
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
        FollowPlayer,
        RenderLayers::layer(1),
    ));
    // Spawn a second camera to render entities in the player's field of view
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
            },
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(entity_render_target),
                ..Default::default()
            },
            ..Default::default()
        },
        FollowPlayer,
        RenderLayers::layer(2),
    ));

    // Spawn a drone that will share FoV with the player
    let size = Extent3d {
        width: 256,
        height: 256,
        ..Default::default()
    };
    image.resize(size);
    let drone_mesh_handle = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
    let drone_render_target = images.add(image.clone());
    let drone_entity_render_target = images.add(image.clone());
    let drone_transform = Transform::from_xyz(179.0, 128.0, 5.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: drone_mesh_handle.clone().into(),
            material: materials.add(ColorMaterial::from(drone_render_target.clone())),
            transform: Transform::from_xyz(0.0, 0.0, 199.198),
            ..Default::default()
        },
        RenderLayers::default().with(3),
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: drone_mesh_handle.clone().into(),
        material: materials.add(ColorMaterial::from(drone_entity_render_target.clone())),
        transform: Transform::from_xyz(0.0, 0.0, 200.198),
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("drone.png"),
            transform: drone_transform.with_rotation(Quat::from_rotation_z(TAU / 1.8)),
            ..Default::default()
        },
        CollisionGroups::new(PLAYER_GROUP, Group::all()),
        FieldOfView {
            view_distance: 128.0,
            view_angle: TAU / 12.0,
            mesh: drone_mesh_handle,
            texture: drone_render_target.clone(),
        },
    ));
    // Spawn a camera that will be used to render the drone's field of view
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
            },
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(drone_render_target),
                ..Default::default()
            },
            transform: drone_transform,
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
    // Spawn a second camera to render entities in the drone's field of view
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
            },
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(drone_entity_render_target),
                ..Default::default()
            },
            transform: drone_transform,
            ..Default::default()
        },
        RenderLayers::layer(2),
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
            RenderLayers::layer(2),
        ));
    }

    // Spawn a background image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("nebula.png"),
            ..Default::default()
        },
        RenderLayers::layer(1), // Layer 1 is where the fov camera looks; hides this from the default camera
    ));
    // Overlay a "fog of war"
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK.with_a(0.85),
            custom_size: Some(Vec2::splat(2048.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..Default::default()
    });

    // Spawn an empty texture we'll draw the "explored" map to
    let size = Extent3d {
        width: 2048,
        height: 2048,
        ..Default::default()
    };
    // This is the texture that the seen map will be rendered to.
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
    image.resize(size);
    let render_target = images.add(image);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: render_target.clone(),
            ..Default::default()
        },
        RenderLayers::default().with(3), // layers 0 and 3 make it visible only to default and "explorer" cameras
    ));
    // Spawn a camera that will be used to reveal the explored map
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            camera: Camera {
                order: -2,
                target: RenderTarget::Image(render_target),
                ..Default::default()
            },
            ..Default::default()
        },
        RenderLayers::layer(3),
    ));
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

        if velocity.length_squared() > 0.0 {
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
                        player_transform.rotate_z(facing.angle_between(to));
                    }
                }
            }
        }
    }
}
