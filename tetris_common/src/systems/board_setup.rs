use bevy::prelude::{BuildChildren, Commands, Entity, SpatialBundle};

use crate::components::{Block, Board, Owned};

pub fn setup_board(mut commands: Commands) {
    let board_entity = commands
        .spawn((Owned, Board::default(), SpatialBundle::default()))
        .id();

    let block_entities: Vec<Entity> = (0usize..10usize)
        .zip(0usize..20usize)
        .map(|(x, y)| {
            commands
                .spawn((
                    Owned,
                    Block {
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
