use bevy::math::Vec2;
use bevy::prelude::Resource;

use tetris_common::components::GridPosition;

#[derive(Resource)]
pub struct BoardUICalculator {
    pub board_position: Vec2,
    pub board_height: usize,
    pub board_width: usize,
    pub block_size: f32,
}

impl BoardUICalculator {
    pub fn new(
        window_position: Vec2,
        block_size: f32,
        board_height: usize,
        board_width: usize,
    ) -> Self {
        Self {
            board_position: window_position,
            board_height,
            board_width,
            block_size,
        }
    }
    pub fn window_relative_position(&self, board_point: &GridPosition) -> Vec2 {
        self.board_position
            + Vec2::new(
                board_point.x as f32 * self.block_size,
                board_point.y as f32 * self.block_size,
            )
    }

    pub fn window_relative_size(&self, board_relative_size: i32) -> f32 {
        board_relative_size as f32 * self.block_size
    }

    /// returns (x, y), (width, height) in Top Bottom Left Right order
    pub fn window_relative_board_walls(&self) -> [(Vec2, Vec2); 4] {
        let wall_stroke = self.block_size;

        let wall_width = self.window_relative_size(self.board_width as i32 + 2);

        let top_pos = self.window_relative_position(&GridPosition {
            x: -1,
            y: self.board_height as i32,
        });
        let top_size = Vec2 {
            x: wall_width,
            y: wall_stroke,
        };

        let bottom_pos = self.window_relative_position(&GridPosition { x: -1, y: -1 });
        let bottom_size = Vec2 {
            x: wall_width,
            y: wall_stroke,
        };

        let wall_height = self.window_relative_size(self.board_height as i32);

        let left_pos = self.window_relative_position(&GridPosition { x: -1, y: 0 });
        let left_size = Vec2 {
            x: wall_stroke,
            y: wall_height,
        };

        let right_pos = self.window_relative_position(&GridPosition {
            x: self.board_width as i32,
            y: 0,
        });
        let right_size = Vec2 {
            x: wall_stroke,
            y: wall_height,
        };

        [
            (top_pos, top_size),
            (bottom_pos, bottom_size),
            (left_pos, left_size),
            (right_pos, right_size),
        ]
    }
}
