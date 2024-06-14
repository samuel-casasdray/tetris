use bevy::prelude::{EventReader, Query, With};

use crate::components::{Owned, Tetromino, TetrominoRotation, TetrominoSpeed};
use crate::events::MovementEvent;

pub fn movement_system(
    mut movement_event: EventReader<MovementEvent>,
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino: Query<&mut Tetromino>,
    mut tetromino_rotation: Query<&mut TetrominoRotation, With<Owned>>,
) {
    let mut next_move = next_move_q.single_mut();

    for movement in movement_event.read() {
        match movement {
            MovementEvent::Right => {
                next_move.x += 1;
            }
            MovementEvent::Left => {
                next_move.x -= 1;
            }
            MovementEvent::Down => {
                next_move.y -= 1;
            }
            MovementEvent::RotationRight => {
                let mut tetromino = tetromino.single_mut();
                let mut tetromino_rotation = tetromino_rotation.single_mut();
                let rot = tetromino.rotation;
                tetromino.rotate_right();
                let (old_position, new_position) = (tetromino.shape.get_blocks(rot), tetromino.shape.get_blocks(tetromino.rotation));
                for i in 0..=old_position.len() {
                    tetromino_rotation.rotations[i] = (new_position[i].x - old_position[i].x, new_position[i].y - old_position[i].y);
                }
            }
            MovementEvent::RotationLeft => {}
        }
    }
}
