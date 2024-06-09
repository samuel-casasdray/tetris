use crate::components::Block;

pub enum Shape {
    I,
    O,
    T,
    L,
    J,
    Z,
    S,
}

impl Shape{
   pub fn get_blocks(&self, mut rotation: u8) -> [Block; 4] {
       rotation %= 4;

       match self {
           Shape::I => match rotation {
               0 => shape_i1(),
               1 => shape_i2(),
               2 => shape_i3(),
               _ => shape_i4(),
           },
           Shape::O => shape_o(),
           Shape::T => match rotation {
               0 => shape_t1(),
               1 => shape_t2(),
               2 => shape_t3(),
               _ => shape_t4(),
           },
           Shape::S => match rotation {
               0 => shape_s1(),
               1 => shape_s2(),
               2 => shape_s3(),
               _ => shape_s4(),
           },
           Shape::Z => match rotation {
               0 => shape_z1(),
               1 => shape_z2(),
               2 => shape_z3(),
               _ => shape_z4(),
           },
           Shape::J => match rotation {
               0 => shape_j1(),
               1 => shape_j2(),
               2 => shape_j3(),
               _ => shape_j4(),
           },
           Shape::L => match rotation {
               0 => shape_l1(),
               1 => shape_l2(),
               2 => shape_l3(),
               _ => shape_l4(),
           },
       }
   }
}

// I
pub fn shape_i1() -> [Block; 4] {
    [
        (0, 2).into(),
        (1, 2).into(),
        (2, 2).into(),
        (3, 2).into(),
    ]
}

pub fn shape_i2() -> [Block; 4] {
    [
        (2, 0).into(),
        (2, 1).into(),
        (2, 2).into(),
        (2, 3).into(),
    ]
}
pub fn shape_i3() -> [Block; 4] {
    [
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
        (3, 1).into(),
    ]
}
pub fn shape_i4() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
        (1, 3).into(),
    ]
}
// T

pub fn shape_t1() -> [Block; 4] {
    [
        (1, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
    ]
}

pub fn shape_t2() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
        (0, 1).into(),
    ]
}

pub fn shape_t3() -> [Block; 4] {
    [
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
        (1, 2).into(),
    ]
}

pub fn shape_t4() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
        (2, 1).into(),
    ]
}

// O
pub fn shape_o() -> [Block; 4] {
    [
        (0, 0).into(),
        (1, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
    ]
}

// S
pub fn shape_s1() -> [Block; 4] {
    [
        (1, 0).into(),
        (2, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
    ]
}

pub fn shape_s2() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (2, 1).into(),
        (2, 2).into(),
    ]
}

pub fn shape_s3() -> [Block; 4] {
    [
        (1, 1).into(),
        (2, 1).into(),
        (0, 2).into(),
        (1, 2).into(),
    ]
}

pub fn shape_s4() -> [Block; 4] {
    [
        (0, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
        (1, 2).into(),
    ]
}

// Z
pub fn shape_z1() -> [Block; 4] {
    [
        (0, 0).into(),
        (1, 0).into(),
        (1, 1).into(),
        (2, 1).into(),
    ]
}

pub fn shape_z2() -> [Block; 4] {
    [
        (2, 0).into(),
        (1, 1).into(),
        (2, 1).into(),
        (1, 2).into(),
    ]
}

pub fn shape_z3() -> [Block; 4] {
    [
        (0, 1).into(),
        (1, 1).into(),
        (1, 2).into(),
        (2, 2).into(),
    ]
}

pub fn shape_z4() -> [Block; 4] {
    [
        (1, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
        (0, 2).into(),
    ]
}

// J
pub fn shape_j1() -> [Block; 4] {
    [
        (0, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
    ]
}

pub fn shape_j2() -> [Block; 4] {
    [
        (1, 0).into(),
        (2, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
    ]
}

pub fn shape_j3() -> [Block; 4] {
    [
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
        (2, 2).into(),
    ]
}

pub fn shape_j4() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (0, 2).into(),
        (1, 2).into(),
    ]
}

// L

pub fn shape_l1() -> [Block; 4] {
    [
        (2, 0).into(),
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
    ]
}

pub fn shape_l2() -> [Block; 4] {
    [
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
        (2, 2).into(),
    ]
}

pub fn shape_l3() -> [Block; 4] {
    [
        (0, 1).into(),
        (1, 1).into(),
        (2, 1).into(),
        (0, 2).into(),
    ]
}

pub fn shape_l4() -> [Block; 4] {
    [
        (0, 0).into(),
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
    ]
}
