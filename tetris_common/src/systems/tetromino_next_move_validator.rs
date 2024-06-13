use bevy::prelude::{Query, With};

use crate::components::{GridPosition, Owned, Tetromino, TetrominoSpeed};

pub fn tetromino_next_move_validator(
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut controlled_shape_q: Query<&mut GridPosition, (With<Owned>, With<Tetromino>)>,
) {
    let mut next_move = next_move_q.single_mut();
    for mut controlled_shape in controlled_shape_q.iter_mut() {
        controlled_shape.x += next_move.x;
        controlled_shape.y += next_move.y;
    }

    *next_move = TetrominoSpeed { x: 0, y: 0 };
}
