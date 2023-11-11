use bevy::prelude::*;

use crate::rand::*;

const TILE_SIZE: f32 = 16.0;
const TILE_Z: f32 = 1.0;
const OFFSET_X: i32 = -32;

pub fn setup_map(mut commands: Commands, mut _world_rng: ResMut<WorldRng>) {
    let mut rooms = Vec::<IRect>::new();

    let ship_length = 64;
    let max_center = 24;
    let room_size = 4..16;

    let mut rng = WyRand::from_entropy();

    for _ in 0..15 {
        let x = rng.gen_range(0..ship_length);
        let size = IVec2::new(
            rng.gen_range(room_size.clone()),
            rng.gen_range(room_size.clone()),
        );
        let mut center = IVec2::new(x, max_center + size.y);

        loop {
            center.y -= 1;
            let new_room = IRect::from_center_size(center, size);
            if new_room.min.y <= 0 {
                break;
            }
            if rooms
                .iter()
                .any(|&room| !room.intersect(new_room).is_empty())
            {
                break;
            }
        }

        // We broke out of the loop because our room intersected something, so back up
        center.y += 1;
        if center.y > max_center {
            // This room doesn't fit here, drop it
            continue;
        }
        let new_room = IRect::from_center_size(center, size);
        println!("Adding new room: {new_room:?}");
        rooms.push(new_room);
        let mut center = new_room.as_rect().center() * TILE_SIZE;
        center.x += OFFSET_X as f32 * TILE_SIZE;
        let size = new_room.as_rect().size() * TILE_SIZE;
        let color = Color::from(rng.gen::<[f32; 3]>()).with_a(0.65);
        // Now spawn the room
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(center.extend(TILE_Z)),
            ..Default::default()
        });
        // And reflect it across the ship's spine
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation((center * Vec2::new(1.0, -1.0)).extend(TILE_Z)),
            ..Default::default()
        });
    }
}

pub fn _ship_hull_by_runs(mut commands: Commands, mut _world_rng: ResMut<WorldRng>) {
    let mut map = vec![0; 64];
    // let size = Vec2::splat(16.0);
    let max_width = 16;
    let min_run = 4;
    let max_run = 8;

    let mut start = 0;
    // let mut rng = world_rng.fork_inner();
    let mut rng = WyRand::from_entropy();

    while start < map.len() {
        let run = rng.gen_range(min_run..max_run).clamp(0, map.len() - start);
        let width = rng.gen_range(0..max_width);

        map[start..(start + run)].copy_from_slice(&vec![width; run]);

        start += run;
    }

    for (x, width) in map.into_iter().enumerate() {
        let x = x as i32 + OFFSET_X;
        // Spawn the ship's spine
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            transform: transform_from_xy(x, 0),
            ..Default::default()
        });

        for y in 0..width {
            let y = y + 1;
            // Spawn ship's hull
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::NAVY,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..Default::default()
                },
                transform: transform_from_xy(x, y),
                ..Default::default()
            });
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::NAVY,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..Default::default()
                },
                transform: transform_from_xy(x, -y),
                ..Default::default()
            });
        }
    }
}

fn transform_from_xy(x: i32, y: i32) -> Transform {
    let pos = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    Transform::from_translation(pos.extend(TILE_Z))
}
