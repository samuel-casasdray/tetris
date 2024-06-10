use bevy::prelude::{Query, With};

use crate::components::{GridPosition, Owned, Tetromino};

pub fn tetromino_gravity_system(
    mut tetrominos_pos: Query<&mut GridPosition, (With<Tetromino>, With<Owned>)>,
) {
    for mut pos in tetrominos_pos.iter_mut() {
        pos.y -= 1;
    }
}
