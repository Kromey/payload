use std::io;
use std::io::prelude::*;
use map::TileMap;

#[derive(Debug)]
enum SectorShape {
    Sphere { r: i32 },
    Cube { w: i32, h: i32 },
    Mesh { w: i32, h: i32 },
    Pyramid { h: i32 },
}

#[derive(Debug)]
pub struct Sector {
    shape: SectorShape,
}

impl Sector {
    pub fn new() -> Sector {
        Sector { shape: SectorShape::Sphere { r: 8 } }
    }

    pub fn print(&self) {
        let mut width = 0;
        let mut height = 0;

        match self.shape {
            SectorShape::Sphere{r} => {
                width = r * 2;
                height = r * 2;
            },
            _ => {
                unimplemented!();
            },
        }

        println!("{} {}", width, height);
    }
}
