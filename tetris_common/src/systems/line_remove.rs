use bevy::prelude::{Commands, Entity, EventReader, Query, With, Without};

use crate::components::{Block, Board, GridPosition, Owned, RelativeGridPosition, Score};
use crate::events::BlockCollisionEvent;

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
            if ys[i] == board.width {
                for (entity, mut pos) in board_blocks_q.iter_mut() {
                    if pos.y > i as i32 {
                        pos.y -= 1;
                    } else if pos.y == i as i32 {
                        commands.entity(entity).despawn();
                    }
                }
                number_of_lines += 1;
            }
        }
        let mut score = score_q.single_mut();
        score.add_score_line(board.level, number_of_lines);
    }
}
