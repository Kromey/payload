use bevy::prelude::*;
use itertools::Itertools;
use petgraph::prelude::UnGraphMap;

use crate::rand::*;

const TILE_SIZE: f32 = 16.0;
const TILE_Z: f32 = 1.0;
const OFFSET: IVec2 = IVec2::new(-32, 0);

#[derive(Debug, Default, Clone, Resource)]
pub struct Rooms {
    rooms: Vec<IRect>,
    graph: UnGraphMap<usize, f32>,
}
impl Rooms {
    fn len(&self) -> usize {
        self.rooms.len()
    }

    fn push(&mut self, new_room: IRect) {
        self.rooms.push(new_room);
        self.graph.add_node(self.rooms.len() - 1);
    }

    fn iter(&self) -> impl Iterator<Item = &IRect> {
        self.rooms.iter()
    }

    fn add_edge(&mut self, p: usize, q: usize, weight: f32) {
        self.graph.add_edge(p, q, weight);
    }
}

pub fn setup_map(mut commands: Commands, mut _world_rng: ResMut<WorldRng>) {
    let mut rooms = Rooms::default();

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
        rooms.rooms.push(new_room);
        rooms.graph.add_node(rooms.len() - 1);
        if center.y > 0 {
            let center = IVec2::new(center.x, -center.y);
            let new_room = IRect::from_center_size(center, size);
            rooms.push(new_room);
        }
    }

    // Calculate Delauney triangulation of the rooms
    let points = rooms
        .iter()
        .map(|room| {
            let center = room.as_rect().center().as_dvec2();
            delaunator::Point {
                x: center.x,
                y: center.y,
            }
        })
        .collect_vec();
    let triangulation = delaunator::triangulate(&points);

    // This is adapted from `forEachTriangleEdge` function at <https://mapbox.github.io/delaunator/>
    // Kudos to "1L-1UX" (illiux#5291) on Roguelikes Discord - Thank you!
    for e in 0..triangulation.triangles.len() {
        let o = triangulation.halfedges[e];
        if e > o || o == delaunator::EMPTY {
            let p = triangulation.triangles[e];
            let q = triangulation.triangles[delaunator::next_halfedge(e)];

            rooms.add_edge(p, q, 1.0);
        }
    }

    // Find adjacent rooms
    let raw_room_list = rooms.rooms.clone();
    for (idx, room) in raw_room_list.iter().enumerate() {
        let adjacency = room.inset(1);
        for (other_idx, other_room) in raw_room_list.iter().enumerate() {
            if other_idx <= idx {
                continue;
            }
            let size = adjacency.intersect(*other_room).size();
            let area = size.x * size.y;
            // If we touch only on a corner, the intersection has area 1 - but we don't care about that
            if area > 1 {
                // Set the weight for this edge to 0 to signify adjacency
                rooms.add_edge(idx, other_idx, 0.0);
            }
        }
    }

    // Spawn rooms
    for room in rooms.iter() {
        let center = room.as_rect().center() * TILE_SIZE + OFFSET.as_vec2() * TILE_SIZE;
        let size = room.as_rect().size() * TILE_SIZE;
        // Ensure we neatly mirror our color for mirrored rooms
        let mut color_rng = seed_rng(room.center().abs());
        let color = Color::from(color_rng.gen::<[f32; 3]>()).with_a(0.65);
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
    }

    commands.insert_resource(rooms);
}

pub fn debug_triangulation(mut gizmos: Gizmos, rooms: Res<Rooms>) {
    for (idx, room) in rooms.iter().enumerate() {
        let start = room.as_rect().center() * TILE_SIZE + OFFSET.as_vec2() * TILE_SIZE;
        for (a, b, weight) in rooms.graph.edges(idx) {
            let to_idx = std::cmp::max(a, b);
            if to_idx <= idx {
                continue;
            }
            let to =
                rooms.rooms[to_idx].as_rect().center() * TILE_SIZE + OFFSET.as_vec2() * TILE_SIZE;
            let color = if *weight == 0.0 {
                Color::GREEN.with_a(0.25)
            } else {
                Color::BLUE.with_a(0.25)
            };
            gizmos.line_2d(start, to, color);
        }
    }
}
