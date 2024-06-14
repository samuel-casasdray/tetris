use bevy::prelude::{Commands, Entity, EventReader, Query, With, Without};
use crate::components::{Block, Board, GridPosition, Owned, RelativeGridPosition};
use crate::events::BlockCollisionEvent;

pub fn line_remove (
    mut commands: Commands,
    current_board: Query<&Board, With<Owned>>,
    mut board_blocks: Query<(Entity, &mut GridPosition), (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
    mut ev_block_collision: EventReader<BlockCollisionEvent>
) {
    for _ in ev_block_collision.read() {
        let board = current_board.single();
        let mut ys = vec![0; board.height];
        for (_, pos) in &board_blocks {
            ys[pos.y as usize] += 1;
        }
        for i in (0..board.height).rev() {
            if ys[i] == board.width {
                for (entity, mut pos) in board_blocks.iter_mut() {
                    if pos.y > i as i32 {
                        pos.y -= 1;
                    } else if pos.y == i as i32 {
                        commands.entity(entity).despawn()
                    }
                }
            }
        }
    }
}