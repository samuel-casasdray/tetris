use bevy::prelude::{BuildChildren, Commands, Entity, SpatialBundle};

use crate::components::{Block, Board, GridPosition, Owned};

pub fn setup_board(mut commands: Commands) {
    let board_entity = commands
        .spawn((Owned, Board::default(), SpatialBundle::default()))
        .id();

    let block_entities: Vec<Entity> = (0usize..10usize)
        .flat_map(|x| (0usize..20usize).map(move |y| (x, y)))
        .map(|(x, y)| {
            commands
                .spawn((
                    Owned,
                    Block,
                    GridPosition {
                        x: x as i32,
                        y: y as i32,
                    },
                    SpatialBundle::default(),
                ))
                .id()
        })
        .collect();

    commands.entity(board_entity).push_children(&block_entities);
}
