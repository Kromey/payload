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

    for _ in 0..25 {
        let x = rng.gen_range(0..ship_length);
        let mut size = IVec2::new(
            rng.gen_range(room_size.clone()),
            rng.gen_range(room_size.clone()),
        );
        let mut center = IVec2::new(x, max_center + size.y);

        loop {
            center.y -= 1;
            let new_room = IRect::from_center_size(center, size);
            if new_room.min.y <= 0 {
                if rng.gen() {
                    // We reached the spine, center this room
                    size.y = (size.y / 2) * 2;
                    center.y = 0;
                } else {
                    // Alternatively, back it off and leave the spine empty
                    center.y += 1;
                }
                break;
            }
            if rooms
                .iter()
                .any(|&room| !room.intersect(new_room).is_empty())
            {
                // We intersected something, so back up
                center.y += 1;
                break;
            }
        }

        if center.y > max_center {
            // This room doesn't fit here, drop it
            continue;
        }
        let new_room = IRect::from_center_size(center, size);
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
        if center.y > 0.0 {
            // And reflect it across the ship's spine
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(
                    (center * Vec2::new(1.0, -1.0)).extend(TILE_Z),
                ),
                ..Default::default()
            });
        }
    }
}
