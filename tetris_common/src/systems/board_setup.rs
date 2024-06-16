use std::time::Duration;

use bevy::prelude::{Commands, SpatialBundle, Timer, TimerMode};

use crate::bundles::OwnedNextMoveBundle;
use crate::components::{Board, GravityTimer, Owned, Score};

pub fn setup_board(mut commands: Commands) {
    commands.spawn(OwnedNextMoveBundle::new());

    commands.spawn((Owned, Board::default(), SpatialBundle::default()));

    commands.spawn(GravityTimer {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });

    commands.spawn(Score::new());
}
