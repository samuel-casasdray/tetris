use bevy::prelude::Bundle;

use crate::board::components::{GridPosition, RelativeGridPosition};
use crate::components::Owned;
use crate::tetromino::components::{Tetromino, TetrominoRotateTo, TetrominoShadow, TetrominoSpeed};

#[derive(Bundle)]
pub struct OwnedTetrominoBundle {
    owned: Owned,
    tetromino: Tetromino,
    tetromino_shadow: TetrominoShadow,
    grid_position: GridPosition,
    grid_position_shadow: RelativeGridPosition,
}

impl OwnedTetrominoBundle {
    pub fn new(grid_position: GridPosition, tetromino: Tetromino) -> Self {
        Self {
            owned: Owned,
            tetromino: tetromino.clone(),
            tetromino_shadow: TetrominoShadow { tetromino },
            grid_position: grid_position.clone(),
            grid_position_shadow: RelativeGridPosition {
                x: grid_position.x,
                y: grid_position.y,
            },
        }
    }

    pub fn new_random(grid_position: GridPosition) -> Self {
        Self::new(grid_position, Tetromino::get_random_shape())
    }
}

#[derive(Bundle)]
pub struct OwnedTetrominoPhysics {
    owned: Owned,
    next_move: TetrominoSpeed,
    next_rotation: TetrominoRotateTo,
}

impl OwnedTetrominoPhysics {
    pub fn new() -> Self {
        Self {
            owned: Owned,
            next_move: TetrominoSpeed { x: 0, y: 0 },
            next_rotation: TetrominoRotateTo(None),
        }
    }
}
