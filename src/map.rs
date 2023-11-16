use bevy::prelude::*;
use itertools::Itertools;
use petgraph::{algo::min_spanning_tree, data::FromElements, prelude::UnGraphMap};

use crate::rand::*;

pub const TILE_SIZE: f32 = 16.0;
const TILE_Z: f32 = 1.0;

#[derive(Debug, Clone, Copy, Resource)]
pub struct ShipParameters {
    pub ship_length: i32,
    pub max_width: i32,
    pub min_rooms: i32,
    pub max_rooms: i32,
    pub room_width_min: i32,
    pub room_width_max: i32,
    pub room_height_min: i32,
    pub room_height_max: i32,
}

impl Default for ShipParameters {
    fn default() -> Self {
        Self {
            ship_length: 64,
            max_width: 24,
            min_rooms: 10,
            max_rooms: 25,
            room_width_min: 4,
            room_width_max: 16,
            room_height_min: 4,
            room_height_max: 16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum EdgeWeight {
    Adjacent,
    Weighted(f32, f32),
}

#[derive(Debug, Default, Clone, Resource)]
pub struct Rooms {
    rooms: Vec<IRect>,
    graph: UnGraphMap<usize, EdgeWeight>,
    mst: UnGraphMap<usize, EdgeWeight>,
}
impl Rooms {
    pub fn len(&self) -> usize {
        self.rooms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }

    fn push(&mut self, new_room: IRect) {
        self.rooms.push(new_room);
        self.graph.add_node(self.rooms.len() - 1);
    }

    pub fn iter(&self) -> impl Iterator<Item = &IRect> {
        self.rooms.iter()
    }

    fn add_edge(&mut self, p: usize, q: usize, weight: EdgeWeight) {
        self.graph.add_edge(p, q, weight);
    }
}

pub fn setup_map(
    mut commands: Commands,
    ship: Res<ShipParameters>,
    mut _world_rng: ResMut<WorldRng>,
) {
    let mut rooms = Rooms::default();

    let mut rng = WyRand::from_entropy();

    for _ in 0..ship.max_rooms {
        let x = rng.gen_range(0..ship.ship_length);
        let mut size = IVec2::new(
            rng.gen_range(ship.room_width_min..ship.room_width_max),
            rng.gen_range(ship.room_height_min..ship.room_height_max),
        );
        let mut center = IVec2::new(x, ship.max_width + size.y);

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

        if center.y > ship.max_width {
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

    // Make sure we got enough rooms
    if rooms.len() < ship.min_rooms as usize {
        return setup_map(commands, ship, _world_rng);
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

            // Weight the links by how far from the spine they are
            // "How far" being the average of the absolute value of their respective y endpoints
            let y = (rooms.rooms[p].center().as_vec2().y.abs()
                + rooms.rooms[q].center().as_vec2().y.abs())
                / 2.0;
            // Additionally favor shorter paths
            let d = rooms.rooms[p]
                .center()
                .as_vec2()
                .distance(rooms.rooms[q].center().as_vec2());

            rooms.add_edge(p, q, EdgeWeight::Weighted(y, d));
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
                // Set the weight for this edge to signify adjacency
                rooms.add_edge(idx, other_idx, EdgeWeight::Adjacent);
            }
        }
    }

    // Calculate MST
    rooms.mst = UnGraphMap::from_elements(min_spanning_tree(&rooms.graph));

    // Spawn rooms
    for room in rooms.iter() {
        let center = room.as_rect().center() * TILE_SIZE;
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
        let start = room.as_rect().center() * TILE_SIZE;
        for (a, b, weight) in rooms.graph.edges(idx) {
            let to_idx = std::cmp::max(a, b);
            if to_idx <= idx {
                continue;
            }
            let to = rooms.rooms[to_idx].as_rect().center() * TILE_SIZE;
            let mut color = match *weight {
                EdgeWeight::Adjacent => Color::GREEN.with_a(0.5),
                EdgeWeight::Weighted(..) => Color::BLUE.with_a(0.5),
            };
            if rooms.mst.contains_edge(idx, to_idx) {
                color = Color::GOLD;
            }
            gizmos.line_2d(start, to, color);
        }
    }
}
