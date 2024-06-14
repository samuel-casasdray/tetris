use bevy::prelude::{Query, With};

use crate::components::{Block, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoRotation, TetrominoSpeed};

pub fn tetromino_next_move_validator(
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut controlled_shape_q: Query<&mut GridPosition, (With<Owned>, With<Tetromino>)>,
    mut tetromino_rotation: Query<&mut TetrominoRotation, With<Owned>>,
    mut controlled_shape_block: Query<&mut RelativeGridPosition, (With<Owned>, With<Block>)>,
) {
    let mut next_move = next_move_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    for mut controlled_shape in controlled_shape_q.iter_mut() {
        controlled_shape.x += next_move.x;
        controlled_shape.y += next_move.y;
    }

    for (mut controlled_shape, (x, y)) in controlled_shape_block.iter_mut().zip(tetromino_rotation.rotations) {
        controlled_shape.x += x;
        controlled_shape.y += y;
    }

    *next_move = TetrominoSpeed { x: 0, y: 0 };
    *tetromino_rotation = TetrominoRotation::new();
}
