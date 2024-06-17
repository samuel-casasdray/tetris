use std::fmt::Display;

use bevy::prelude::{Color, Component, Timer};
use rand::Rng;

use crate::shapes::{Shape, ShapePosition};

pub const DEFAULT_BOARD_WIDTH: usize = 10;
pub const DEFAULT_BOARD_HEIGHT: usize = 20;

#[derive(Debug, Component)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub enable_gravity: bool,
    pub level: u8,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            height: DEFAULT_BOARD_HEIGHT,
            width: DEFAULT_BOARD_WIDTH,
            enable_gravity: true,
            level: 0,
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Tetromino {
    pub rotation: u8,
    pub shape: Shape,
}

#[derive(Debug, Component, Clone)]
pub struct TetrominoShadow {
    pub tetromino: Tetromino
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

/// An entity controlled by the user running the app
#[derive(Debug, Component)]
pub struct Owned;

/// Particularly used to create a fake shape to test collision against
#[derive(Debug, Component)]
pub struct Fake;

#[derive(Debug, Component)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/// A position inside the tetris board grid
#[derive(Debug, Component, Clone)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for GridPosition {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Component, Clone)]
pub struct RelativeGridPosition {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for RelativeGridPosition {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

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

#[derive(Debug, Component)]
pub struct TetrominoSpeed {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component)]
pub struct TetrominoRotateTo(pub Option<u8>);

#[derive(Debug, Component)]
pub struct Score {
    pub score: u64,
}

#[derive(Component)]
pub struct ScoreText;

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }
    pub fn add_score_line(&mut self, level: u8, lines: u8) {
        self.score += match lines {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            _ => 1200,
        } * (level + 1) as u64;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score : {}", self.score)
    }
}
