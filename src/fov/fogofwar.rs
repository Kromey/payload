use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

use super::Viewable;

pub fn setup_fog_of_war(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // Spawn a background image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("nebula.png"),
            ..Default::default()
        },
        Viewable::Static,
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
