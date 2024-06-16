use crate::shapes::ShapePosition;

pub fn shape_z1() -> ShapePosition {
    [(0, 2).into(), (1, 2).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_z2() -> ShapePosition {
    [(2, 2).into(), (2, 1).into(), (1, 1).into(), (1, 0).into()]
}

pub fn shape_z3() -> ShapePosition {
    [(2, 0).into(), (1, 0).into(), (1, 1).into(), (0, 1).into()]
}

pub fn shape_z4() -> ShapePosition {
    [(0, 0).into(), (0, 1).into(), (1, 1).into(), (1, 2).into()]
}