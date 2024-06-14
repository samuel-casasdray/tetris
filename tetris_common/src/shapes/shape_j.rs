use crate::shapes::ShapePosition;

pub fn shape_j1() -> ShapePosition {
    [(0, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_j2() -> ShapePosition {
    [(1, 0).into(), (2, 0).into(), (1, 1).into(), (1, 2).into()]
}

pub fn shape_j3() -> ShapePosition {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (2, 2).into()]
}

pub fn shape_j4() -> ShapePosition {
    [(1, 0).into(), (1, 1).into(), (0, 2).into(), (1, 2).into()]
}
