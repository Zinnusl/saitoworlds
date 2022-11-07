use crate::pos::*;
use web_sys::CanvasRenderingContext2d;

pub trait Renderable {
    fn render(&self, _context: &CanvasRenderingContext2d, _offset: Offset) {
    }
}
