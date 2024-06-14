use std::time::Duration;

use bevy::prelude::{Commands, SpatialBundle, Timer, TimerMode};

use crate::bundles::OwnedNextMoveBundle;
use crate::components::{Board, GravityTimer, Owned};

pub fn setup_board(mut commands: Commands) {
    commands.spawn(OwnedNextMoveBundle::new());
    
    commands .spawn((Owned, Board::default(), SpatialBundle::default()));

    commands.spawn(GravityTimer {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });

    // let block_entities: Vec<Entity> = (0usize..10usize)
    //     .flat_map(|x| (0usize..20usize).map(move |y| (x, y)))
    //     .map(|(x, y)| {
    //         commands
    //             .spawn(OwnedBlockBundle::new(GridPosition {
    //                 x: x as i32,
    //                 y: y as i32,
    //             }))
    //             .id()
    //     })
    //     .collect();
    //
    // commands.entity(board_entity).push_children(&block_entities);
}
