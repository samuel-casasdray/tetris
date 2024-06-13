use bevy::prelude::Bundle;

use crate::components::{
    Block, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoSpeed,
};

#[derive(Bundle)]
pub struct OwnedTetrominoBundle {
    owned: Owned,
    tetromino: Tetromino,
    grid_position: GridPosition,
}

impl OwnedTetrominoBundle {
    pub fn new(grid_position: GridPosition, tetromino: Tetromino) -> Self {
        Self {
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
    pub fn new(relative_grid_position: RelativeGridPosition) -> Self {
        Self {
            relative_grid_position,
            owned_block_bundle: OwnedBlockBundle::new((0, 0).into()),
        }
    }
}

#[derive(Bundle)]
pub struct OwnedBlockBundle {
    owned: Owned,
    block: Block,
    grid_position: GridPosition,
}
impl OwnedBlockBundle {
    pub fn new(grid_position: GridPosition) -> Self {
        Self {
            owned: Owned,
            block: Block,
            grid_position,
        }
    }
}
#[derive(Bundle)]
pub struct OwnedNextMoveBundle {
    owned: Owned,
    next_move: TetrominoSpeed,
}
impl OwnedNextMoveBundle {
    pub fn new() -> Self {
        Self {
            owned: Owned,
            next_move: TetrominoSpeed { x: 0, y: 0 },
        }
    }
}
