use crate::components::Block;

pub fn shape_i1() -> [Block; 4] {
    [(0, 2).into(), (1, 2).into(), (2, 2).into(), (3, 2).into()]
}

pub fn shape_i2() -> [Block; 4] {
    [(2, 0).into(), (2, 1).into(), (2, 2).into(), (2, 3).into()]
}
pub fn shape_i3() -> [Block; 4] {
    [(0, 1).into(), (1, 1).into(), (2, 1).into(), (3, 1).into()]
}
pub fn shape_i4() -> [Block; 4] {
    [(1, 0).into(), (1, 1).into(), (1, 2).into(), (1, 3).into()]
}
