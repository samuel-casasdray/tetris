use bevy::prelude::{Query, Res, Time, With};

use crate::components::{GravityTimer, GridPosition, Owned, Tetromino};

pub fn tetromino_gravity_system(
    mut tetrominos_pos: Query<&mut GridPosition, (With<Tetromino>, With<Owned>)>,
    time: Res<Time>,
    mut gravity_timer: Query<&mut GravityTimer>,
) {
    let mut gravity_timer = gravity_timer.single_mut();
    gravity_timer.timer.tick(time.delta());

    if gravity_timer.timer.finished() {
        for mut pos in tetrominos_pos.iter_mut() {
            pos.y -= 1;
        }
    }
}
