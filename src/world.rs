use crate::pos::*;
use std::collections::HashMap;
use crate::displayinfo::*;
use crate::clickable::*;
use web_sys::CanvasRenderingContext2d;
use crate::renderable::*;
use std::cell::RefCell;
use std::rc::Rc;

mod tile;

use tile::*;

#[derive(Clone, PartialEq)]
pub struct World {
    camera: Offset,
    hexes: HashMap<Pos, Tile>
}

pub type WorldPtr = Rc<RefCell<World>>;

impl World {

    pub fn new() -> WorldPtr {
        let mut terrain = HashMap::new();
        fill_terrain_circle(&mut terrain, 20, Offset::WorldSpace(Pos::new(0, 0)));
        Rc::new(RefCell::new(World {
            camera: Offset::ScreenSpace((0.0, 0.0)),
            // camera: Pos::new(2000, 2000),
            hexes: terrain
        }))
    }
    pub fn move_camera_to_mut(&mut self, pos: Offset) {
        self.camera = pos;
    }
    pub fn move_camera_offset_mut(&mut self, pos: Offset) {
        self.camera = self.camera + pos;
    }
}

impl Renderable for World {
    fn render(&self, context: &CanvasRenderingContext2d, offset: Offset) {
        let canvas = context.canvas().unwrap();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        self.hexes.iter().for_each(|(_pos, hex)| {
            hex.render(context, self.camera + offset);
        });
    }
}

impl Clickable for World {
    fn was_clicked(&self, _: Option<&World>, _x: f64, _y: f64) -> bool {
        true
    }

    fn clicked_mut(&mut self, _world: Option<&World>, info: &mut Option<DisplayInfo>, x: f64, y: f64) {
        let view_world = self.clone();
        self.hexes.iter_mut().for_each(|(_pos, hex)| {
            if hex.was_clicked(Some(&view_world), x, y) {
                hex.clicked_mut(Some(&view_world), info, x, y);
            }
        });
    }
}

fn fill_terrain_circle(terrain: &mut HashMap<Pos, Tile>, size: usize, offset: Offset) {
    let pos_from_offset = Pos::from(offset);

    terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Iceberg, pos_from_offset));

    for i in 1..=size {
        let mut offset = offset + Offset::new((i as i64)*2, 0);
        for _ in 0..i {
            offset = offset.nw();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Iceberg, pos_from_offset));
        }
        for _ in 0..i {
            offset = offset.w();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Volcano, pos_from_offset));
        }
        for _ in 0..i {
            offset = offset.sw();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Iceberg, pos_from_offset));
        }
        for _ in 0..i {
            offset = offset.so();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Volcano, pos_from_offset));
        }
        for _ in 0..i {
            offset = offset.o();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Iceberg, pos_from_offset));
        }
        for _ in 0..i {
            offset = offset.no();
            let pos_from_offset = Pos::from(offset);
            terrain.insert(pos_from_offset, Tile::new_with_pos(TileImg::Volcano, pos_from_offset));
        }
    }
}
