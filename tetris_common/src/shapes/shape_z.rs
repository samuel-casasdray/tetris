use crate::components::RelativeGridPosition;

pub fn shape_z1() -> [RelativeGridPosition; 4] {
    [(0, 0).into(), (1, 0).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_z2() -> [RelativeGridPosition; 4] {
    [(2, 0).into(), (1, 1).into(), (2, 1).into(), (1, 2).into()]
}

pub fn shape_z3() -> [RelativeGridPosition; 4] {
    [(0, 1).into(), (1, 1).into(), (1, 2).into(), (2, 2).into()]
}

pub fn shape_z4() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (0, 1).into(), (1, 1).into(), (0, 2).into()]
}
