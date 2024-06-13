use bevy::prelude::{EventReader, Query, With};

use crate::components::{Owned, TetrominoSpeed};
use crate::events::MovementEvent;

pub fn movement_system(
    mut movement_event: EventReader<MovementEvent>,
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
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
            MovementEvent::RotationRight => {}
            MovementEvent::RotationLeft => {}
        }
    }
}
