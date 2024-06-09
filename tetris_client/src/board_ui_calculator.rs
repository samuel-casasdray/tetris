use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

// TODO: Bouger ce component dans la common lib
#[derive(Component, Clone)]
pub struct BoardPoint(pub i32, pub i32);

#[derive(Resource)]
pub struct BoardUICalculator {
    board_position: Vec2,
    board_height: usize,
    board_width: usize,
    block_size: f32,
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
    pub fn window_relative_position(&self, board_point: &BoardPoint) -> Vec2 {
        self.board_position + Vec2::new(board_point.0 as f32 * self.block_size, board_point.1 as f32 * self.block_size)
    }

    pub fn window_relative_size(&self, board_relative_size: i32) -> f32 {
        board_relative_size as f32 * self.block_size
    }

    /// returns (x, y), (width, height) in Top Bottom Left Right order
    pub fn window_relative_board_walls(&self) -> [(Vec2, Vec2); 4] {
        let wall_stroke = self.block_size;

        let wall_width = self.window_relative_size(self.board_width as i32 + 2);

        let top_pos = self.window_relative_position(&BoardPoint(-1, self.board_height as i32));
        let top_size = Vec2 { x: wall_width, y: wall_stroke };

        let bottom_pos = self.window_relative_position(&BoardPoint(-1, -1));
        let bottom_size = Vec2 { x: wall_width, y: wall_stroke };

        let wall_height = self.window_relative_size(self.board_height as i32);

        let left_pos = self.window_relative_position(&BoardPoint(-1, 0));
        let left_size = Vec2 { x: wall_stroke, y: wall_height };

        let right_pos = self.window_relative_position(&BoardPoint(self.board_width as i32, 0));
        let right_size = Vec2 { x: wall_stroke, y: wall_height };

        [
            (top_pos, top_size),
            (bottom_pos, bottom_size),
            (left_pos, left_size),
            (right_pos, right_size)
        ]
    }
}