use crate::components::RelativeGridPosition;

pub fn shape_s1() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (2, 0).into(), (0, 1).into(), (1, 1).into()]
}

pub fn shape_s2() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (2, 1).into(), (2, 2).into()]
}

pub fn shape_s3() -> [RelativeGridPosition; 4] {
    [(1, 1).into(), (2, 1).into(), (0, 2).into(), (1, 2).into()]
}

pub fn shape_s4() -> [RelativeGridPosition; 4] {
    [(0, 0).into(), (0, 1).into(), (1, 1).into(), (1, 2).into()]
}
