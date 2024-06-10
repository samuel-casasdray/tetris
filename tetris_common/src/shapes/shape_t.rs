use crate::components::GridPosition;

pub fn shape_t1() -> [GridPosition; 4] {
    [(1, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_t2() -> [GridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 1).into()]
}

pub fn shape_t3() -> [GridPosition; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (1, 2).into()]
}

pub fn shape_t4() -> [GridPosition; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (2, 1).into()]
}
