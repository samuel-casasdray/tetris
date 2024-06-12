use bevy::prelude::{Query, With};

use crate::components::{GridPosition, NextMove, Owned, Tetromino};

pub fn tetromino_next_move_validator(
    mut next_move_q: Query<&mut NextMove, With<Owned>>,
    mut controlled_shape_q: Query<&mut GridPosition, (With<Owned>, With<Tetromino>)>,
) {
    let mut next_move = next_move_q.single_mut();
    for mut controlled_shape in controlled_shape_q.iter_mut() {
        controlled_shape.x += next_move.0.x;
        controlled_shape.y += next_move.0.y;
    }

    next_move.0 = (0, 0).into();
}
