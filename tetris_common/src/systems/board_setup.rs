use bevy::prelude::Commands;
use crate::components::{Board, Owned};

pub fn setup_board(mut command: Commands) {
    command.spawn((Owned, Board {
        width: 20,
        height: 10,
    }));
}
