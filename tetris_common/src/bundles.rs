use bevy::prelude::{Bundle, Color, Name};

use crate::components::{Block, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoRotation, TetrominoSpeed};

#[derive(Bundle)]
pub struct OwnedTetrominoBundle {
    name:Name,
    owned: Owned,
    tetromino: Tetromino,
    grid_position: GridPosition,
}

impl OwnedTetrominoBundle {
    pub fn new(grid_position: GridPosition, tetromino: Tetromino) -> Self {
        Self {
            name: Name::new("OwnedTetromino"),
            owned: Owned,
            tetromino,
            grid_position,
        }
    }

    pub fn new_random(grid_position: GridPosition) -> Self {
        Self::new(grid_position, Tetromino::get_random_shape())
    }
}

#[derive(Bundle)]
pub struct OwnedRelativeBlockBundle {
    owned_block_bundle: OwnedBlockBundle,
    relative_grid_position: RelativeGridPosition,
}

impl OwnedRelativeBlockBundle {
    pub fn new(relative_grid_position: RelativeGridPosition, color: Color) -> Self {
        Self {
            relative_grid_position,
            owned_block_bundle: OwnedBlockBundle::new((0, 0).into(), color),
        }
    }
}

#[derive(Bundle)]
pub struct OwnedBlockBundle {
    name: Name,
    owned: Owned,
    block: Block,
    grid_position: GridPosition,
}

impl OwnedBlockBundle {
    pub fn new(grid_position: GridPosition, color: Color) -> Self {
        Self {
            name: Name::new("OwnedBlock"),
            owned: Owned,
            block: Block::new(color),
            grid_position,
        }
    }
}

#[derive(Bundle)]
pub struct OwnedTetrominoSpeedBundle {
    name: Name,
    owned: Owned,
    next_move: TetrominoSpeed,
    next_rotation: TetrominoRotation,
}

impl OwnedTetrominoSpeedBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("Owned Tetromino Speed"),
            owned: Owned,
            next_move: TetrominoSpeed { x: 0, y: 0 },
            next_rotation: TetrominoRotation::new(),
        }
    }
}
