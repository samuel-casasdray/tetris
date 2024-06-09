use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

#[derive(Component, Clone)]
pub struct BoardPoint(pub i32, pub i32);

#[derive(Resource)]
pub struct BoardCalculator {
    board_position: Vec2,
    block_size: f32,
}

impl BoardCalculator {
    pub fn new(window_position: Vec2, block_size: f32) -> Self {
        Self {
            board_position: window_position,
            block_size,
        }
    }
    pub fn window_relative_position(&self, board_point: &BoardPoint) -> Vec2 {
        self.board_position + Vec2::new(board_point.0 as f32 * self.block_size, board_point.1 as f32 * self.block_size)
    }

    pub fn window_relative_size(&self, board_relative_size: i32) -> f32 {
        board_relative_size as f32 * self.block_size
    }
}