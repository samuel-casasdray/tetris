use bevy::prelude::{Query, Res, Time, With};

use crate::components::{GravityTimer, NextMove, Owned};

pub fn tetromino_gravity_system(
    time: Res<Time>,
    mut gravity_timer_q: Query<&mut GravityTimer>,
    mut next_move_q: Query<&mut NextMove, With<Owned>>,
) {
    let mut gravity_timer = gravity_timer_q.single_mut();
    gravity_timer.timer.tick(time.delta());

    if gravity_timer.timer.finished() {
        let mut next_move = next_move_q.single_mut();
        next_move.0.y -= 1;
    }
}
