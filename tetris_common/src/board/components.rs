use bevy::prelude::{Color, Component};

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
