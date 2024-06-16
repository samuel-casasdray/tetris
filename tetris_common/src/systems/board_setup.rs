use std::time::Duration;

use bevy::prelude::{BuildChildren, Commands, SpatialBundle, Timer, TimerMode};

use crate::bundles::OwnedTetrominoPhysics;
use crate::components::{Board, GravityTimer, Owned, Score};

pub fn setup_board(mut commands: Commands) {
    commands
        .spawn((Owned, Board::default(), SpatialBundle::default()))
        .with_children(|builder| {
            builder.spawn(OwnedTetrominoPhysics::new());
            builder.spawn(GravityTimer {
                timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            });
            builder.spawn(Score::new());
        });
}
