use bevy::prelude::{Children, EventWriter, info, Query, With, Without};

use crate::components::{
    Block, Board, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoRotateTo,
    TetrominoSpeed,
};
use crate::events::BlockCollisionEvent;
use crate::systems::relative_position::get_grid_position;

pub fn collision_resolver(
    board_q: Query<&Board, With<Owned>>,
    tetromino_q: Query<(&Tetromino, &GridPosition, &Children), With<Owned>>,
    board_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
    shape_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, With<RelativeGridPosition>)>,
    mut tetromino_speed_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
) {
    let (tetromino, tetromino_position, tetromino_children) = match tetromino_q.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    let mut tetromino_speed = tetromino_speed_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    // Get the tetromino's blocks with the rotation applied if any
    let tetromino_blocks: Vec<GridPosition> = if let Some(rotation) = tetromino_rotation.0 {
        let relative_blocks = tetromino.shape.get_blocks(rotation);
        relative_blocks
            .iter()
            .map(|block| get_grid_position(block, tetromino_position))
            .collect()
    } else {
        shape_blocks_q
            .iter_many(tetromino_children)
            .cloned()
            .collect()
    };

    let board = board_q.single();
    for shape_block in tetromino_blocks.iter() {
        let next_x = shape_block.x + tetromino_speed.x;
        let next_y = shape_block.y + tetromino_speed.y;

        // Check collision with bottom wall
        if next_y < 0 {
            tetromino_speed.y = 0;
            tetromino_rotation.0 = None;
            info!("Bottom wall collision");
            ev_block_collision.send(BlockCollisionEvent);
            return;
        }

        // Check collision with left/right wall
        if next_x >= board.width as i32 || next_x < 0 {
            info!("Left/Right wall collision");
            tetromino_speed.x = 0;
            tetromino_rotation.0 = None;
        }
    }

    for block in board_blocks_q.iter() {
        for shape_block in tetromino_blocks.iter() {
            let next_x = shape_block.x + tetromino_speed.x;
            let next_y = shape_block.y + tetromino_speed.y;

            // Check collision with blocks
            if block.x == next_x && block.y == next_y {
                info!("block collision in {:?} {:?}", block, shape_block);
                tetromino_speed.x = 0;
                tetromino_rotation.0 = None;
            }

            // Check collision with blocks when shape comes from the top
            if block.y == next_y && shape_block.y > block.y && block.x == next_x {
                tetromino_speed.x = 0;
                tetromino_speed.y = 0;
                tetromino_rotation.0 = None;
                info!("Block collision from top in {:?} {:?}", block, shape_block);
                ev_block_collision.send(BlockCollisionEvent);
                return;
            }
        }
    }
}
