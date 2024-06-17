use std::cmp::min;

use bevy::prelude::{Query, With, Without};

use crate::components::{Block, DEFAULT_BOARD_HEIGHT, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoShadow};
use crate::systems::relative_position::get_grid_position;

pub fn shadow_movement(
    tetromino: Query<(&Tetromino, &GridPosition), With<Owned>>,
    mut tetromino_shadow: Query<(&mut TetrominoShadow, &mut RelativeGridPosition), With<Owned>>,
    board_blocks: Query<&GridPosition, (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
) {
    // If the tetromino exist
    let tetromino_exists = !tetromino.is_empty();
    if tetromino_exists {
        // Copy of tetromino into his shadow
        let tetromino = tetromino.single();
        let mut tetromino_shadow = tetromino_shadow.single_mut();
        tetromino_shadow.0.tetromino = tetromino.0.clone();

        // Place the shadow at the bottom
        let positions = tetromino_shadow.0.tetromino.get_blocks_positions();
        let mut grid_positions: [GridPosition; 4] = [
            get_grid_position(&positions[0], tetromino.1),
            get_grid_position(&positions[1], tetromino.1),
            get_grid_position(&positions[2], tetromino.1),
            get_grid_position(&positions[3], tetromino.1)
        ];
        let mut min_y = DEFAULT_BOARD_HEIGHT as i32;
        for grid_position in &grid_positions {
            min_y = min(min_y, grid_position.y);
        }
        for grid_position in grid_positions.as_mut() {
            grid_position.y -= min_y;
        }

        // Test if collision with a block and if yes the shadow goes up
        let mut new_min_y = min_y;
        for block_position in &board_blocks {
            for shape_position in &grid_positions {
                if shape_position.x == block_position.x &&
                    block_position.y > shape_position.y &&
                    min_y - block_position.y < new_min_y
                {
                    new_min_y = min_y - block_position.y;
                }
            }
        }
        for grid_position in grid_positions.as_mut() {
            grid_position.y += min_y - new_min_y;
        }
        
        // We update the shadow position
        tetromino_shadow.1.y -= new_min_y;
    } else {
        println!("{}", tetromino_shadow.is_empty());
    }
    
}