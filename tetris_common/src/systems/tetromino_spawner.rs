use bevy::prelude::{BuildChildren, Commands, Query, SpatialBundle, With};

use crate::bundles::{OwnedRelativeBlockBundle, OwnedTetrominoBundle};
use crate::components::{Board, GridPosition, Owned, Tetromino};

pub fn tetromino_spawner(
    mut commands: Commands,
    q_tetromino: Query<&Tetromino, With<Owned>>,
    board_q: Query<&Board, With<Owned>>,
) {
    let tetromino_exists = !q_tetromino.is_empty();
    if tetromino_exists {
        return;
    }

    let tetromino = Tetromino::get_random_shape();
    let color = tetromino.shape.color();
    let positions = tetromino.get_blocks_positions();
    let board = board_q.single();

    commands
        .spawn((
            OwnedTetrominoBundle::new(
                GridPosition {
                    x: 5,
                    y: board.height as i32,
                },
                tetromino,
            ),
            SpatialBundle::default(),
        ))
        .with_children(|child| {
            for relative_positions in positions {
                child.spawn((
                    OwnedRelativeBlockBundle::new(relative_positions, color),
                    SpatialBundle::default(),
                ));
            }
        });
}
