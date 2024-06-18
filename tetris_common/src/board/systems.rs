use std::time::Duration;

use bevy::hierarchy::{BuildChildren, Parent};
use bevy::prelude::{
    Commands, Entity, EventReader, Query, SpatialBundle, Timer, TimerMode, With, Without,
};

use crate::bundles::OwnedTetrominoPhysics;
use crate::components::{
    Block, Board, GravityTimer, GridPosition, Owned, RelativeGridPosition, Score,
};
use crate::tetromino::events::BlockCollisionEvent;

pub fn setup_board(mut commands: Commands) {
    commands
        .spawn((Owned, Board::default(), SpatialBundle::default()))
        .with_children(|builder| {
            builder.spawn(OwnedTetrominoPhysics::new());
            builder.spawn(GravityTimer {
                timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            });
            builder.spawn((Score::new(), Owned));
        });
}

pub fn line_remove(
    mut commands: Commands,
    current_board_q: Query<&Board, With<Owned>>,
    mut board_blocks_q: Query<
        (Entity, &mut GridPosition),
        (With<Block>, With<Owned>, Without<RelativeGridPosition>),
    >,
    mut ev_block_collision: EventReader<BlockCollisionEvent>,
    mut score_q: Query<&mut Score>,
) {
    for _ in ev_block_collision.read() {
        let board = current_board_q.single();
        let mut ys = vec![0; board.height];
        for (_, pos) in &board_blocks_q {
            if pos.y < board.height as i32 {
                ys[pos.y as usize] += 1;
            }
        }
        let mut number_of_lines = 0;
        for i in (0..board.height).rev() {
            if ys[i] != board.width {
                continue;
            }
            for (entity, mut pos) in board_blocks_q.iter_mut() {
                if pos.y > i as i32 {
                    pos.y -= 1;
                } else if pos.y == i as i32 {
                    commands.entity(entity).despawn();
                }
            }
            number_of_lines += 1;
        }
        let mut score = score_q.single_mut();
        score.add_score_line(board.level, number_of_lines);
    }
}

/// This system update all chidrens GridPosition relative to their parent and their RelativePosition component.
/// Carefull this system only work at 1 level deep
pub fn relative_position_system(
    positions: Query<(Entity, &RelativeGridPosition, &Parent), With<GridPosition>>,
    mut grid_positions: Query<&mut GridPosition>,
) {
    let entities_with_rel_pos_grid_pos: Vec<_> =
        positions
            .iter()
            .map(|(entity, relative_pos, parent)| {
                (
                    entity,
                    relative_pos,
                    grid_positions.get(parent.get()).expect(
                        "To use a relative position the parent must have a GridPosition component",
                    ).clone(),
                )
            })
            .collect();

    for (entity, relative_pos, parent_grid_pos) in entities_with_rel_pos_grid_pos {
        let mut grid_position = grid_positions.get_mut(entity).unwrap();
        *grid_position = get_grid_position(&relative_pos, &parent_grid_pos);
    }
}

pub fn get_grid_position(
    relative_position: &RelativeGridPosition,
    parent_position: &GridPosition,
) -> GridPosition {
    GridPosition::from((
        parent_position.x + relative_position.x,
        parent_position.y + relative_position.y,
    ))
}
