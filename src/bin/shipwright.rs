use std::cmp::{max, min};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use payload::{
    camera::MainCamera,
    map::{Rooms, ShipParameters},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum ShipState {
    #[default]
    Creating,
    Displaying,
}

#[derive(Debug, Clone, Copy, Resource)]
struct ShipStatistics {
    parameters: ShipParameters,
    ship_length: i32,
    ship_width: i32,
    room_count: usize,
}

#[derive(Debug, Default, Clone, Copy, Resource)]
struct ShipSeed {
    use_seed: bool,
    value: u64,
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
    mut ship: ResMut<ShipParameters>,
    statistics: Res<ShipStatistics>,
    mut seed: ResMut<ShipSeed>,
) {
    egui::SidePanel::left("shipwright_panel")
        .exact_width(200.0)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Ship Parameters");

            ui.checkbox(&mut seed.use_seed, "Use Seed");
            let mut seed_str = seed.value.to_string();
            ui.text_edit_singleline(&mut seed_str);
            if let Ok(val) = seed_str.parse::<u64>() {
                seed.value = val;
            }
            if seed.use_seed {
                ship.seed = Some(seed.value);
            } else {
                ship.seed = None;
            }

            ui.separator();
            ui.heading("Ship Size");
            ui.add(egui::Slider::new(&mut ship.ship_length, 16..=256).text("Length"));
            ui.add(egui::Slider::new(&mut ship.max_width, 16..=64).text("Max Width"));
            ui.add(egui::Slider::new(&mut ship.min_rooms, 0..=64).text("Min Rooms"));
            ui.add(egui::Slider::new(&mut ship.max_rooms, 8..=64).text("Max Rooms"));

            ui.separator();
            ui.heading("Room Size");
            ui.add(egui::Slider::new(&mut ship.room_width_min, 4..=64).text("Min Width"));
            ui.add(egui::Slider::new(&mut ship.room_width_max, 4..=64).text("Max Width"));
            ui.add(egui::Slider::new(&mut ship.room_height_min, 4..=64).text("Min Height"));
            ui.add(egui::Slider::new(&mut ship.room_height_max, 4..=64).text("Max Height"));

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Generate Ship").clicked() {
                    next_state.set(ShipState::Creating);
                }

                if ui
                    .add_enabled(
                        *ship != ShipParameters::default(),
                        egui::Button::new("Reset Defaults"),
                    )
                    .clicked()
                {
                    *ship = ShipParameters::default();
                    seed.use_seed = false;
                }
            });
        });

    egui::Window::new("Ship Statistics")
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Ship Statistics");
            ui.label(format!("Length: {}", statistics.ship_length));
            ui.label(format!("Width: {}", statistics.ship_width));
            ui.label(format!("Rooms: {}", statistics.room_count));

            ui.heading("Parameters");
            let seed_str = statistics.parameters.seed.unwrap().to_string();
            if seed_str.len() > 10 {
                let short_seed = &seed_str[..10];
                if ui
                    .add(
                        egui::Label::new(format!("Seed: {short_seed}..."))
                            .sense(egui::Sense::click()),
                    )
                    .on_hover_text(seed_str)
                    .clicked()
                {
                    seed.use_seed = true;
                    seed.value = statistics.parameters.seed.unwrap();
                };
            } else {
                ui.label(format!("Seed: {seed_str}"));
            }
            ui.label(format!("Length: {}", statistics.parameters.ship_length));
            ui.label(format!("Width: {}", statistics.parameters.max_width));
            ui.label(format!(
                "Rooms: {} - {}",
                statistics.parameters.min_rooms, statistics.parameters.max_rooms
            ));
            ui.label(format!(
                "Room Width: {} - {}",
                statistics.parameters.room_width_min, statistics.parameters.room_width_max
            ));
            ui.label(format!(
                "Room Height: {} - {}",
                statistics.parameters.room_height_min, statistics.parameters.room_height_max
            ));
        });
}

fn shipwright_input(
    mut camera_qry: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut scroll_evt: EventReader<MouseWheel>,
    mut motion_evt: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut contexts: EguiContexts,
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
            if mouse_buttons.pressed(MouseButton::Left) && !contexts.ctx_mut().is_using_pointer() {
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

fn gather_ship_stats(
    mut commands: Commands,
    parameters: Res<ShipParameters>,
    rooms: Res<Rooms>,
    mut seed: ResMut<ShipSeed>,
) {
    if let Some(ship_seed) = parameters.seed {
        seed.value = ship_seed;
    }

    let (min_x, min_y, max_x, max_y) = rooms
        .iter()
        .map(|room| (room.min.x, room.min.y, room.max.x, room.max.y))
        .reduce(|acc, room| {
            (
                min(acc.0, room.0),
                min(acc.1, room.1),
                max(acc.2, room.2),
                max(acc.3, room.3),
            )
        })
        .unwrap();
    let ship_length = max_x - min_x;
    let ship_width = max_y - min_y;

    let stats = ShipStatistics {
        parameters: *parameters,
        ship_length,
        ship_width,
        room_count: rooms.len(),
    };

    commands.insert_resource(stats);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(payload::rand::RandPlugin::default())
        .add_plugins(EguiPlugin)
        .init_resource::<ShipParameters>()
        .init_resource::<ShipSeed>()
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
                shipwright_ui.run_if(in_state(ShipState::Displaying)),
                shipwright_input,
                payload::map::setup_map.run_if(in_state(ShipState::Creating)),
            ),
        )
        .add_systems(OnEnter(ShipState::Creating), cleanup_sprites)
        .add_systems(
            OnEnter(ShipState::Displaying),
            (center_camera, gather_ship_stats),
        )
        .run();
}
