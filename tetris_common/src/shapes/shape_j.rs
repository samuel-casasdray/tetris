use crate::components::Block;

pub fn shape_j1() -> [Block; 4] {
    [(0, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_j2() -> [Block; 4] {
    [(1, 0).into(), (2, 0).into(), (1, 1).into(), (1, 2).into()]
}

pub fn shape_j3() -> [Block; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (2, 2).into()]
}

pub fn shape_j4() -> [Block; 4] {
    [(1, 0).into(), (1, 1).into(), (0, 2).into(), (1, 2).into()]
}
