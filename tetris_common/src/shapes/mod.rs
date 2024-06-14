use crate::components::RelativeGridPosition;
use crate::shapes::shape_i::{shape_i1, shape_i2, shape_i3, shape_i4};
use crate::shapes::shape_j::{shape_j1, shape_j2, shape_j3, shape_j4};
use crate::shapes::shape_l::{shape_l1, shape_l2, shape_l3, shape_l4};
use crate::shapes::shape_o::shape_o;
use crate::shapes::shape_s::{shape_s1, shape_s2, shape_s3, shape_s4};
use crate::shapes::shape_t::{shape_t1, shape_t2, shape_t3, shape_t4};
use crate::shapes::shape_z::{shape_z1, shape_z2, shape_z3, shape_z4};

mod shape_i;
mod shape_j;
mod shape_l;
mod shape_o;
mod shape_s;
mod shape_t;
mod shape_z;

#[derive(Debug, Clone)]
pub enum Shape {
    I,
    O,
    T,
    L,
    J,
    Z,
    S,
}



pub type ShapePosition = [RelativeGridPosition; 4];

impl Shape {
    pub fn get_blocks(&self, mut rotation: u8) -> ShapePosition {
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

    // fn rotate_shape(&self, rotation: u8) -> ShapePosition {
    //     let base_shape = shape_z();
    //     let center: (i32, i32) = (base_shape.1.x, base_shape.1.y);
    //     return [(0, 0).into(), (0, 0).into(), (0, 0).into(), (0, 0).into()]
    // }
    // 
    // fn rotation(x: i32, y: i32, center_x: i32, center_y: i32, rotation: u8) -> (i32, i32) {
    //     match rotation { 
    //         0 => (x, y),
    //         1 => (-y, x),
    //         2 => (2 * center_x - x, 2 * center_y - y),
    //         _ => ()
    //     }
    // }
}
