use crate::shapes::ShapePosition;

pub fn shape_t1() -> ShapePosition {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (1, 2).into()]
}

pub fn shape_t2() -> ShapePosition {
    [(1, 2).into(), (1, 1).into(), (1, 0).into(), (2, 1).into()]
}

pub fn shape_t3() -> ShapePosition {
    [(2, 1).into(), (1, 1).into(), (0, 1).into(), (1, 0).into()]
}

pub fn shape_t4() -> ShapePosition {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 1).into()]
}
