use bevy::prelude::Bundle;

use crate::board::components::GridPosition;
use crate::tetromino::components::{Tetromino, TetrominoShadow};

#[derive(Bundle)]
pub struct TetrominoBundle {
    tetromino: Tetromino,
    tetromino_shadow: TetrominoShadow,
    grid_position: GridPosition,
}

impl TetrominoBundle {
    pub fn new(grid_position: GridPosition, tetromino: Tetromino) -> Self {
        Self {
            tetromino: tetromino.clone(),
            tetromino_shadow: TetrominoShadow { tetromino },
            grid_position: grid_position.clone(),
        }
    }

    pub fn new_random(grid_position: GridPosition) -> Self {
        Self::new(grid_position, Tetromino::get_random_shape())
    }
}
