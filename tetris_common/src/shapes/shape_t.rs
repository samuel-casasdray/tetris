use crate::components::Block;

pub fn shape_t1() -> [Block; 4] {
    [(1, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()]
}

pub fn shape_t2() -> [Block; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 1).into()]
}

pub fn shape_t3() -> [Block; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (1, 2).into()]
}

pub fn shape_t4() -> [Block; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (2, 1).into()]
}
