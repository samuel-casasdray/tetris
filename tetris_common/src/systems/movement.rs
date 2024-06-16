use bevy::prelude::{EventReader, Query, With};

use crate::components::{Owned, Tetromino, TetrominoRotateTo, TetrominoSpeed};
use crate::events::MovementEvent;

pub fn movement_system(
    mut movement_event: EventReader<MovementEvent>,
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino: Query<&mut Tetromino, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
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
                let mut rotation = tetromino_rotation.single_mut();
                let tetromino = tetromino.single_mut();
                rotation.0 = Some(tetromino.get_rotation_right());
            }
            MovementEvent::RotationLeft => {
                let mut rotation = tetromino_rotation.single_mut();
                let tetromino = tetromino.single_mut();
                rotation.0 = Some(tetromino.get_rotation_left());
            }
        }
    }
}
