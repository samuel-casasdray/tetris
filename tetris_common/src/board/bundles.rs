use bevy::prelude::{Bundle, Color};

use crate::board::components::{Block, GridPosition, RelativeGridPosition};
use crate::components::Owned;

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
    owned: Owned,
    block: Block,
    grid_position: GridPosition,
}

impl OwnedBlockBundle {
    pub fn new(grid_position: GridPosition, color: Color) -> Self {
        Self {
            owned: Owned,
            block: Block::new(color),
            grid_position,
        }
    }
}
