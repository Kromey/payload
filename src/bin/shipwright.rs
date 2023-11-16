use bevy::{
    app::AppExit,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use payload::{camera::MainCamera, map::Rooms};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum ShipState {
    #[default]
    Creating,
    Displaying,
}

fn advance_state(state: Res<State<ShipState>>, mut next_state: ResMut<NextState<ShipState>>) {
    match *state.get() {
        ShipState::Creating => next_state.set(ShipState::Displaying),
        ShipState::Displaying => {}
    }
}

fn shipwright_ui(
    mut next_state: ResMut<NextState<ShipState>>,
    mut contexts: EguiContexts,
    mut exit: EventWriter<AppExit>,
) {
    egui::SidePanel::left("shipwright_panel")
        .exact_width(200.0)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Shipwright");
            ui.separator();
            if ui.button("New Ship").clicked() {
                next_state.set(ShipState::Creating);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::TOP), |ui| {
                if ui.button("Exit").clicked() {
                    exit.send(AppExit);
                }
                ui.separator();
            });
        });
}

fn shipwright_input(
    mut camera_qry: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut scroll_evt: EventReader<MouseWheel>,
    mut motion_evt: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    // Zoom in/out with scroll wheel
    let zoom: f32 = scroll_evt
        .read()
        .map(|wheel| match wheel.unit {
            bevy::input::mouse::MouseScrollUnit::Line => wheel.y * 10.0, // totally arbitrary, I dunno!
            bevy::input::mouse::MouseScrollUnit::Pixel => wheel.y,
        })
        .sum();

    // Pan with mouse motion
    let pan: Vec2 = motion_evt
        .read()
        .map(|motion| {
            if mouse_buttons.pressed(MouseButton::Left) {
                Vec2::new(-motion.delta.x, motion.delta.y)
            } else {
                Vec2::ZERO
            }
        })
        .sum();

    // Now update the camera
    for (mut transform, mut projection) in camera_qry.iter_mut() {
        transform.translation += pan.extend(0.0);
        projection.scale = (projection.scale + zoom * 0.01).clamp(0.2, 2.0);
    }
}

fn cleanup_sprites(mut commands: Commands, sprites_qry: Query<Entity, With<Sprite>>) {
    for sprite in sprites_qry.iter() {
        commands.entity(sprite).despawn_recursive();
    }
}

fn center_camera(mut camera_qry: Query<&mut Transform, With<MainCamera>>, rooms: Res<Rooms>) {
    // Find the x center of the ship
    let x_min = rooms.iter().map(|room| room.min.x).min().unwrap();
    let x_max = rooms.iter().map(|room| room.max.x).max().unwrap();
    let x_center = (x_min + x_max) as f32 / 2.0 * payload::map::TILE_SIZE;

    for mut camera in camera_qry.iter_mut() {
        camera.translation.x = x_center - 150.0; // Account for the sidebar's width; why isn't this the same??
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(payload::rand::RandPlugin::default())
        .add_plugins(EguiPlugin)
        .add_state::<ShipState>()
        .add_systems(
            Startup,
            (
                payload::camera::spawn_camera,
                payload::sprites::load_sprites,
            ),
        )
        .add_systems(
            Update,
            (
                payload::map::debug_triangulation.run_if(in_state(ShipState::Displaying)),
                advance_state,
                shipwright_ui,
                shipwright_input,
                payload::map::setup_map.run_if(in_state(ShipState::Creating)),
            ),
        )
        .add_systems(OnEnter(ShipState::Creating), cleanup_sprites)
        .add_systems(OnEnter(ShipState::Displaying), center_camera)
        .run();
}
