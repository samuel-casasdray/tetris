use crate::components::RelativeGridPosition;

pub fn shape_t1() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_t2() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 1).into()]
}

pub fn shape_t3() -> [RelativeGridPosition; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (1, 2).into()]
}

pub fn shape_t4() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (2, 1).into()]
}
