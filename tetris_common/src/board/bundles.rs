use bevy::prelude::{Bundle, Color};

use crate::board::components::{Block, GridPosition, RelativeGridPosition};

#[derive(Bundle)]
pub struct RelativeBlockBundle {
    owned_block_bundle: BlockBundle,
    relative_grid_position: RelativeGridPosition,
}

impl RelativeBlockBundle {
    pub fn new(relative_grid_position: RelativeGridPosition, color: Color) -> Self {
        Self {
            relative_grid_position,
            owned_block_bundle: BlockBundle::new((0, 0).into(), color),
        }
    }
}

#[derive(Bundle)]
pub struct BlockBundle {
    block: Block,
    grid_position: GridPosition,
}

impl BlockBundle {
    pub fn new(grid_position: GridPosition, color: Color) -> Self {
        Self {
            block: Block::new(color),
            grid_position,
        }
    }
}
