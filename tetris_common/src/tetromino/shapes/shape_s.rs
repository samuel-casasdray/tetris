use crate::tetromino::shapes::ShapePosition;

pub fn shape_s1() -> ShapePosition {
    [(0, 1).into(), (1, 1).into(), (1, 2).into(), (2, 2).into()]
}

pub fn shape_s2() -> ShapePosition {
    [(1, 2).into(), (1, 1).into(), (2, 1).into(), (2, 0).into()]
}

pub fn shape_s3() -> ShapePosition {
    [(2, 1).into(), (1, 1).into(), (1, 0).into(), (0, 0).into()]
}

pub fn shape_s4() -> ShapePosition {
    [(1, 0).into(), (1, 1).into(), (0, 1).into(), (0, 2).into()]
}
