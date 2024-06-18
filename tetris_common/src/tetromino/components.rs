use bevy::prelude::{Component, Timer};
use rand::Rng;

use crate::tetromino::shapes::{Shape, ShapePosition};

#[derive(Debug, Component, Clone)]
pub struct Tetromino {
    pub rotation: u8,
    pub shape: Shape,
}

#[derive(Debug, Component, Clone)]
pub struct TetrominoShadow {
    pub tetromino: Tetromino,
}

impl Tetromino {
    pub fn get_blocks_positions(&self) -> ShapePosition {
        self.shape.get_blocks(self.rotation)
    }

    pub fn rotate_left(&mut self) {
        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }
    }

    pub fn get_rotation_left(&self) -> u8 {
        if self.rotation == 0 {
            3
        } else {
            self.rotation - 1
        }
    }

    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn get_rotation_right(&self) -> u8 {
        (self.rotation + 1) % 4
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

        Self { rotation: 0, shape }
    }
}

#[derive(Debug, Component)]
pub struct TetrominoSpeed {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component)]
pub struct TetrominoRotateTo(pub Option<u8>);

#[derive(Debug, Component)]
pub struct GravityTimer {
    pub timer: Timer,
}

#[derive(Debug, Component)]
pub struct MovementTimer {
    pub timer: Timer,
}

#[derive(Debug, Component)]
pub struct RotationTimer {
    pub timer: Timer,
}
