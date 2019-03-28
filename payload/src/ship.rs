use std::io;
use std::io::prelude::*;

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
        let mut max_x = 0;
        let mut max_y = 0;

        match self.shape {
            SectorShape::Sphere{r} => {
                max_x = r * 2;
                max_y = r * 2;
            },
            _ => {
                unimplemented!();
            },
        }

        println!("{} {}", max_x, max_y);
    }
}
