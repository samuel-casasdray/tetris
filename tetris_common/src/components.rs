use bevy::prelude::Component;
use rand::Rng;

use crate::shapes::Shape;

#[derive(Component)]
pub struct Board {
    pub height: usize,
    pub width: usize,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            height: 20,
            width: 10,
        }
    }
}

#[derive(Component)]
pub struct Tetromino {
    rotation: u8,
    shape: Shape,
}

impl Tetromino {
    pub fn get_blocks(&self) ->[Block; 4] {
        self.shape.get_blocks(self.rotation)
    }

    pub fn rotate_left(&mut self) {
        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }
    }

    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }
    pub fn get_random_shape() -> Self {
        let mut rng = rand::thread_rng();
        let shape = match rng.gen_range(0..=6) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            _ => Shape::L,
        };
        
        Self{
            rotation: 0,
            shape,
        }
    }
}

/// An entity controlled by the user running the app
#[derive(Component)]
pub struct Owned;

/// Particularly used to create a fake shape to test collision against
#[derive(Component)]
pub struct Fake;

#[derive(Component)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Block {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x,
            y,
        }
    }
}

/// A position inside the tetris board grid
#[derive(Component, Clone)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for GridPosition {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x,
            y,
        }
    }
}
