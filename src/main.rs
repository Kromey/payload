extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Orthographic3;
use amethyst::core::Transform;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
    Flipped, PngFormat, Texture, TextureMetadata, TextureHandle,
    SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, SpriteRender,
    Camera, Projection, ScreenDimensions,
};
use amethyst::ui::UiBundle;
use amethyst::utils::application_root_dir;

pub fn load_texture<N>(name: N, world: &World) -> TextureHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

fn load_sprite_sheet<N>(name: N, world: &World, texture_handle: TextureHandle) -> SpriteSheetHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        SpriteSheetFormat,
        texture_handle,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

fn init_image(world: &mut World, texture_handle: &TextureHandle) {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);

    world
        .create_entity()
        .with(transform)
        .with(texture_handle.clone())
        .with(Flipped::Horizontal)
        .build();
}

#[derive(Debug, Default)]
struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let texture_handle = load_texture("tilesheet.png", &data.world);

        let sprite_sheet_handle = load_sprite_sheet("tilesheet.ron", &data.world, texture_handle);

        self.initialize_sprite(&mut data.world, sprite_sheet_handle);
        self.initialize_camera(&mut data.world);
    }
}

impl ExampleState {
    fn initialize_sprite(
        &mut self,
        world: &mut World,
        sprite_sheet_handle: SpriteSheetHandle,
    ) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width() as u32, dim.height() as u32)
        };

        for x in 0..width/32 {
            for y in 0..height/32 {
                //Place the sprite
                let mut sprite_transform = Transform::default();
                sprite_transform.set_xyz(x as f32 * 32., y as f32 * 32., 0.);

                let sprite = (x + y) % 9;

                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: sprite as usize,
                };

                world
                    .create_entity()
                    .with(sprite_render)
                    .with(sprite_transform)
                    .build();
            }
        }
    }

    fn initialize_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = Transform::default();
        transform.set_xyz(0., 0., 10.);

        world
            .create_entity()
            .with(transform)
            .with(Camera::from(Projection::Orthographic(Orthographic3::new(
                            0.0,
                            width,
                            0.0,
                            height,
                            0.0,
                            20.0,
            ))))
            .build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir();
    let display_config = DisplayConfig::load(format!(
        "{}/resources/display_config.ron",
        app_root
    ));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.], 1.)
            .with_pass(
                DrawFlat2D::new()
                    .with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor())?

        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?;
    let assets_directory = format!("{}/assets/", app_root);
    let mut game = Application::new(assets_directory, ExampleState::default(), game_data)?;
    game.run();

    Ok(())
}
