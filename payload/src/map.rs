use std::default::Default;
use std::ops::{Index, IndexMut};

#[derive(Debug,Clone)]
enum TileType {
    Space,
    Ship,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Space
    }
}


#[derive(Debug,Clone,Default)]
struct Tile {
    tile_type: TileType,
}


#[derive(Debug)]
struct TileMap {
    data: Box<[Tile]>,
    width: usize,
    height: usize,
    depth: usize,
}

impl TileMap {
    pub fn new(width: usize, height: usize, depth: usize) -> TileMap {
        TileMap {
            data: vec![Default::default(); width * height * depth].into_boxed_slice(),
            width,
            height,
            depth,
        }
    }

    #[inline]
    fn calc_offset(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }
}

impl Index<[usize; 3]> for TileMap {
    type Output = Tile;

    fn index(&self, index: [usize; 3]) -> &Tile {
        &self.data[self.calc_offset(index[0], index[1], index[2])]
    }
}
impl IndexMut<[usize; 3]> for TileMap {
    fn index_mut(&mut self, index: [usize; 3]) -> &mut Tile {
        &mut self.data[self.calc_offset(index[0], index[1], index[2])]
    }
}

impl Index<[usize; 2]> for TileMap {
    type Output = Tile;

    fn index(&self, index: [usize; 2]) -> &Tile {
        &self.data[self.calc_offset(index[0], index[1], 0)]
    }
}
impl IndexMut<[usize; 2]> for TileMap {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Tile {
        &mut self.data[self.calc_offset(index[0], index[1], 0)]
    }
}
