use bevy::prelude::{Query, With};

use crate::components::{
    Block, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoRotateTo, TetrominoSpeed,
};

pub fn tetromino_next_move_validator(
    mut tetromino_speed_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
    mut controlled_shape_position_q: Query<(&mut GridPosition, &mut Tetromino), With<Owned>>,
    mut controlled_shape_block: Query<&mut RelativeGridPosition, (With<Owned>, With<Block>)>,
) {
    let mut next_move = tetromino_speed_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    let (mut tetromino_position, mut tetromino) = controlled_shape_position_q.single_mut();
    tetromino_position.x += next_move.x;
    tetromino_position.y += next_move.y;

    if let Some(rotation) = tetromino_rotation.0 {
        tetromino.rotation = rotation;
        let blocks = tetromino.shape.get_blocks(rotation);
        for (index, mut tetromino_block_position) in controlled_shape_block.iter_mut().enumerate() {
            *tetromino_block_position = blocks[index].clone();
        }
    }

    *next_move = TetrominoSpeed { x: 0, y: 0 };
    *tetromino_rotation = TetrominoRotateTo(None);
}
