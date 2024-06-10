use bevy::math::Vec2;
use bevy::prelude::Resource;

use tetris_common::components::GridPosition;


pub const DEFAULT_BOARD_WIDTH: usize = 10;
pub const DEFAULT_BOARD_HEIGHT: usize = 20;
pub const MAX_BOARD_WIDTH_PERCENT: f32 = 0.4;
pub const MAX_BOARD_HEIGHT_PERCENT: f32 = 0.9;

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
    pub fn set_window_position(&mut self, width: f32, height: f32) -> Vec2 {
        self.board_position = get_window_position(self.block_size, width, height);
        self.board_position
    }
     
}
pub fn get_window_position(block_size: f32, width: f32, height: f32) -> Vec2 {
    let x = (width - block_size * (DEFAULT_BOARD_WIDTH + 2) as f32) / 2.;
    let y = (height - block_size * DEFAULT_BOARD_HEIGHT as f32) / 2.;
    Vec2::new(x, y)
}