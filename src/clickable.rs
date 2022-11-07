use crate::world::*;
use crate::displayinfo::*;
use std::rc::Rc;

pub trait Clickable {
    fn was_clicked(&self, _world: Option<&World>, _x: f64, _y: f64) -> bool {
        false
    }
    fn clicked_mut(&mut self, _world: Option<&World>, _info: &mut Option<DisplayInfo>, _x: f64, _y: f64) {
    }
}
