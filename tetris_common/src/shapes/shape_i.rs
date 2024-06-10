use crate::components::RelativeGridPosition;

pub fn shape_i1() -> [RelativeGridPosition; 4] {
    [(0, 2).into(), (1, 2).into(), (2, 2).into(), (3, 2).into()]
}

pub fn shape_i2() -> [RelativeGridPosition; 4] {
    [(2, 0).into(), (2, 1).into(), (2, 2).into(), (2, 3).into()]
}
pub fn shape_i3() -> [RelativeGridPosition; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (3, 1).into()]
}
pub fn shape_i4() -> [RelativeGridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (1, 3).into()]
}
