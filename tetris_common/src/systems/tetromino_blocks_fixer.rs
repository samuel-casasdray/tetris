use bevy::prelude::{BuildChildren, Children, Commands, Entity, EventReader, Query, With};

use crate::components::{Block, Board, Owned, RelativeGridPosition, Tetromino, TetrominoShadow};
use crate::events::BlockCollisionEvent;

pub fn tetromino_blocks_fixer(
    mut commands: Commands,
    tetromino_q: Query<(Entity, &Children), (With<Tetromino>, With<Owned>)>,
    blocks_q: Query<Entity, (With<Block>, With<Owned>, With<RelativeGridPosition>)>,
    board_q: Query<Entity, (With<Owned>, With<Board>)>,
    mut block_collision_event: EventReader<BlockCollisionEvent>,
) {
    for _ in block_collision_event.read() {
        if let Ok((entity, children)) = tetromino_q.get_single() {
            for child in children {
                if let Ok(block_entity) = blocks_q.get(*child) {
                    let board_entity = board_q.single();
                    commands
                        .entity(block_entity)
                        .remove::<RelativeGridPosition>()
                        .remove_parent()
                        .set_parent(board_entity);
                }
            }
            commands.entity(entity).remove::<TetrominoShadow>().despawn();
        }
    }
}
