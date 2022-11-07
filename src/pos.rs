
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64
}
impl Default for Pos {
    fn default() -> Self {
        Pos {
            x: 0,
            y: 0
        }
    }
}
impl From<Offset> for Pos {
    fn from(offset: Offset) -> Self {
        match offset {
            Offset::WorldSpace(Pos{x, y}) => Pos {
                x: x as i64,
                y: y as i64
            },
            Offset::ScreenSpace((x, y)) => Pos {
                x: x as i64/32,
                y: y as i64/32
            }
        }
    }
}

pub trait RelativePos {
    fn nw(&self) -> Offset;
    fn w(&self) -> Offset;
    fn sw(&self) -> Offset;
    fn no(&self) -> Offset;
    fn o(&self) -> Offset;
    fn so(&self) -> Offset;
}

impl RelativePos for Pos {
    fn nw(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::nw())
    }
    fn w(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::w())
    }
    fn sw(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::sw())
    }
    fn no(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::no())
    }
    fn o(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::o())
    }
    fn so(&self) -> Offset {
        Offset::WorldSpace(*self + Offset::so())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Offset {
    ScreenSpace((f64, f64)),
    WorldSpace(Pos)
}
impl RelativePos for Offset {
    fn nw(&self) -> Offset {
        *self + Offset::nw()
    }
    fn w(&self) -> Offset {
        *self + Offset::w()
    }
    fn sw(&self) -> Offset {
        *self + Offset::sw()
    }
    fn no(&self) -> Offset {
        *self + Offset::no()
    }
    fn o(&self) -> Offset {
        *self + Offset::o()
    }
    fn so(&self) -> Offset {
        *self + Offset::so()
    }
}
impl Offset {
    pub fn new(x: i64, y: i64) -> Offset {
        Offset::WorldSpace(Pos::new(x, y))
    }
    fn nw() -> Offset {
        Offset::new(-1, -1)
    }
    fn w() -> Offset {
        Offset::new(-2, 0)
    }
    fn sw() -> Offset {
        Offset::new(-1, 1)
    }
    fn no() -> Offset {
        Offset::new(1, -1)
    }
    fn o() -> Offset {
        Offset::new(2, 0)
    }
    fn so() -> Offset {
        Offset::new(1, 1)
    }
}

impl Pos {
    pub fn new_origin() -> Pos {
        Pos {
            x: 0,
            y: 0
        }
    }
    pub fn new(x: i64, y: i64) -> Pos {
        Pos {
            x,
            y
        }
    }
}
impl std::ops::Add<Offset> for Pos {
    type Output = Pos;
    fn add(self, other: Offset) -> Pos {
        match other {
            Offset::WorldSpace(pos) => Pos {
                x: self.x + pos.x,
                y: self.y + pos.y
            },
            Offset::ScreenSpace(offset) => Pos {
                x: self.x + (offset.0/32.0) as i64,
                y: self.y + (offset.1/48.0) as i64
            }
        }
    }
}
impl std::ops::Add<Offset> for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Offset {
        match self {
            Offset::ScreenSpace((x, y)) => {
                match rhs {
                    Offset::ScreenSpace((x2, y2)) => Offset::ScreenSpace((x + x2, y + y2)),
                    Offset::WorldSpace(Pos { x: x2, y: y2 }) => Offset::ScreenSpace((x + x2 as f64 * 32.0, y + y2 as f64 * 48.0))
                }
            },
            Offset::WorldSpace(Pos { x, y }) => {
                match rhs {
                    Offset::ScreenSpace((x2, y2)) => Offset::ScreenSpace((x as f64 * 32.0 + x2, y as f64 * 48.0 + y2)),
                    Offset::WorldSpace(Pos { x: x2, y: y2 }) => Offset::WorldSpace(Pos::new(x + x2, y + y2))
                }
            }
        }
    }
}
impl std::ops::Neg for Offset {
    type Output = Offset;
    fn neg(self) -> Offset {
        match self {
            Offset::ScreenSpace((x, y)) => Offset::ScreenSpace((-x, -y)),
            Offset::WorldSpace(Pos { x, y }) => Offset::WorldSpace(Pos::new(-x, -y))
        }
    }
}
