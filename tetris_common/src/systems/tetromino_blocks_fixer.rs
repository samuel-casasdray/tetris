use bevy::prelude::{Children, Commands, Entity, EventReader, Query, With};

use crate::components::{Block, Owned, Tetromino};
use crate::events::BlockCollisionEvent;

pub fn tetromino_blocks_fixer(
    mut commands: Commands,
    tetromino_q: Query<(Entity, &Children), (With<Tetromino>, With<Owned>)>,
    mut blocks_q: Query<&mut Block, With<Owned>>,
    mut block_collision_event: EventReader<BlockCollisionEvent>,
) {
    for _ in block_collision_event.read() {
        if let Ok((entity, children)) = tetromino_q.get_single() {
            commands.entity(entity).despawn();
        }
    }
}
