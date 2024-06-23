use std::cmp::min;

use bevy::hierarchy::Children;
use bevy::log::{debug, info};
use bevy::prelude::{
    BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, SpatialBundle, Time,
    With, Without,
};

use crate::board::bundles::RelativeBlockBundle;
use crate::board::components::{
    Block, Board, DEFAULT_BOARD_HEIGHT, GridPosition, RelativeGridPosition,
};
use crate::board::systems::get_grid_position;
use crate::tetromino::bundles::TetrominoBundle;
use crate::tetromino::components::{GravityTimer, Tetromino, TetrominoShadow};
use crate::tetromino::events::{
    BlockCollisionEvent, MovementEvent, TetrominoMovementCheckedEvent, TetrominoMovementEvent,
};

pub fn tetromino_spawner(
    mut commands: Commands,
    q_tetromino: Query<&Tetromino>,
    board_q: Query<&Board>,
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
            TetrominoBundle::new(
                GridPosition {
                    x: 3,
                    y: board.height as i32,
                },
                tetromino,
            ),
            SpatialBundle::default(),
        ))
        .with_children(|child| {
            for relative_positions in positions {
                child.spawn((
                    RelativeBlockBundle::new(relative_positions, color),
                    SpatialBundle::default(),
                ));
            }
        });
}

pub fn tetromino_next_move_validator(
    mut controlled_shape_position_q: Query<(&mut GridPosition, &mut Tetromino)>,
    mut controlled_shape_block: Query<&mut RelativeGridPosition, With<Block>>,
    mut new_tetromino_position_ev: EventReader<TetrominoMovementCheckedEvent>,
) {
    for TetrominoMovementCheckedEvent {
        relative_position,
        rotation,
    } in new_tetromino_position_ev.read()
    {
        let next_move = relative_position.clone().unwrap_or_default();
        let tetromino_rotation = rotation;
        let (mut tetromino_position, mut tetromino) = controlled_shape_position_q.single_mut();
        tetromino_position.x += next_move.x;
        tetromino_position.y += next_move.y;

        if let Some(rotation) = tetromino_rotation {
            tetromino.rotation = *rotation;
            let blocks = tetromino.shape.get_blocks(*rotation);
            for (index, mut tetromino_block_position) in
                controlled_shape_block.iter_mut().enumerate()
            {
                *tetromino_block_position = blocks[index].clone();
            }
        }
    }
}

pub fn tetromino_blocks_fixer(
    mut commands: Commands,
    tetromino_q: Query<(Entity, &Children), With<Tetromino>>,
    blocks_q: Query<Entity, (With<Block>, With<RelativeGridPosition>)>,
    board_q: Query<Entity, With<Board>>,
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
            commands
                .entity(entity)
                .remove::<TetrominoShadow>()
                .despawn();
        }
    }
}
pub fn shadow_movement(
    tetromino: Query<(&Tetromino, &GridPosition)>,
    mut tetromino_shadow: Query<(&mut TetrominoShadow, &mut RelativeGridPosition)>,
    board_blocks: Query<&GridPosition, (With<Block>, Without<RelativeGridPosition>)>,
) {
    // If the tetromino exist
    let tetromino_exists = !tetromino.is_empty();
    if tetromino_exists {
        // Copy of tetromino into his shadow
        let tetromino = tetromino.single();
        let mut tetromino_shadow = tetromino_shadow.single_mut();
        tetromino_shadow.0.tetromino = tetromino.0.clone();

        // Place the shadow at the bottom
        let positions = tetromino_shadow.0.tetromino.get_blocks_positions();
        let mut grid_positions: [GridPosition; 4] = [
            get_grid_position(&positions[0], tetromino.1),
            get_grid_position(&positions[1], tetromino.1),
            get_grid_position(&positions[2], tetromino.1),
            get_grid_position(&positions[3], tetromino.1),
        ];
        let mut min_y = DEFAULT_BOARD_HEIGHT as i32;
        for grid_position in &grid_positions {
            min_y = min(min_y, grid_position.y);
        }
        for grid_position in grid_positions.as_mut() {
            grid_position.y -= min_y;
        }

        // Test if collision with a block and if yes the shadow goes up
        let mut new_min_y = min_y;
        for block_position in &board_blocks {
            for shape_position in &grid_positions {
                if shape_position.x == block_position.x
                    && block_position.y > shape_position.y
                    && min_y - block_position.y < new_min_y
                {
                    new_min_y = min_y - block_position.y;
                }
            }
        }
        for grid_position in grid_positions.as_mut() {
            grid_position.y += min_y - new_min_y;
        }

        // We update the shadow position
        tetromino_shadow.1.y -= new_min_y;
    } else {
        println!("{}", tetromino_shadow.is_empty());
    }
}

pub fn movement_system(
    mut movement_event_reader: EventReader<MovementEvent>,
    mut tetromino: Query<&mut Tetromino>,
    mut new_tetromino_movement_writer: EventWriter<TetrominoMovementEvent>,
) {
    for movement in movement_event_reader.read() {
        match movement {
            MovementEvent::Right => {
                new_tetromino_movement_writer.send(TetrominoMovementEvent {
                    relative_position: Some(RelativeGridPosition { x: 1, y: 0 }),
                    rotation: None,
                });
            }
            MovementEvent::Left => {
                new_tetromino_movement_writer.send(TetrominoMovementEvent {
                    relative_position: Some(RelativeGridPosition { x: -1, y: 0 }),
                    rotation: None,
                });
            }
            MovementEvent::Down => {
                new_tetromino_movement_writer.send(TetrominoMovementEvent {
                    relative_position: Some(RelativeGridPosition { x: 0, y: -1 }),
                    rotation: None,
                });
            }
            MovementEvent::RotationRight => {
                let tetromino = tetromino.single_mut();
                new_tetromino_movement_writer.send(TetrominoMovementEvent {
                    relative_position: None,
                    rotation: Some(tetromino.get_rotation_right()),
                });
            }
            MovementEvent::RotationLeft => {
                let tetromino = tetromino.single_mut();
                new_tetromino_movement_writer.send(TetrominoMovementEvent {
                    relative_position: None,
                    rotation: Some(tetromino.get_rotation_left()),
                });
            }
        }
    }
}

pub fn tetromino_gravity_system(
    time: Res<Time>,
    mut gravity_timer_q: Query<&mut GravityTimer>,
    board_q: Query<&Board>,
    mut new_tetromino_position: EventWriter<TetrominoMovementEvent>,
) {
    if !board_q.single().enable_gravity {
        return;
    }

    let mut gravity_timer = gravity_timer_q.single_mut();
    gravity_timer.timer.tick(time.delta());

    if gravity_timer.timer.finished() {
        new_tetromino_position.send(TetrominoMovementEvent {
            relative_position: Some(RelativeGridPosition { x: 0, y: -1 }),
            rotation: None,
        });
    }
}

pub fn collision_resolver(
    board_q: Query<&Board>,
    tetromino_q: Query<(&Tetromino, &GridPosition, &Children)>,
    board_blocks_q: Query<&GridPosition, (With<Block>, Without<RelativeGridPosition>)>,
    shape_blocks_q: Query<&GridPosition, (With<Block>, With<RelativeGridPosition>)>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
    mut tetromino_movement_ev: EventReader<TetrominoMovementEvent>,
    mut new_tetromino_position_ev: EventWriter<TetrominoMovementCheckedEvent>,
) {
    let (tetromino, tetromino_position, tetromino_children) = match tetromino_q.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    'event_loop: for TetrominoMovementEvent {
        relative_position,
        rotation,
    } in tetromino_movement_ev.read()
    {
        let mut new_position_to_send = TetrominoMovementCheckedEvent {
            relative_position: relative_position.clone(),
            rotation: *rotation,
        };
        // Get the tetromino's blocks with the rotation applied if any
        let tetromino_blocks: Vec<GridPosition> = if let Some(rotation) = rotation {
            let relative_blocks = tetromino.shape.get_blocks(*rotation);
            relative_blocks
                .iter()
                .map(|block| get_grid_position(block, tetromino_position))
                .collect()
        } else {
            shape_blocks_q
                .iter_many(tetromino_children)
                .cloned()
                .collect()
        };

        let board = board_q.single();
        for shape_block in tetromino_blocks.iter() {
            let (next_x, next_y) = if let Some(position) = relative_position {
                (shape_block.x + position.x, shape_block.y + position.y)
            } else {
                (shape_block.x, shape_block.y)
            };

            // Check collision with bottom wall
            if next_y < 0 {
                if let Some(p) = new_position_to_send.relative_position.as_mut() {
                    p.y = 0;
                }
                new_position_to_send.rotation = None;
                info!("Bottom wall collision");
                ev_block_collision.send(BlockCollisionEvent);
                continue 'event_loop;
            }

            // Check collision with left/right wall
            if next_x >= board.width as i32 || next_x < 0 {
                info!("Left/Right wall collision");
                if let Some(p) = new_position_to_send.relative_position.as_mut() {
                    p.x = 0;
                }
                new_position_to_send.rotation = None;
            }
        }

        for block in board_blocks_q.iter() {
            for shape_block in tetromino_blocks.iter() {
                let (next_x, next_y) = if let Some(position) = relative_position {
                    (shape_block.x + position.x, shape_block.y + position.y)
                } else {
                    (shape_block.x, shape_block.y)
                };

                // Check collision with blocks
                if block.x == next_x && block.y == next_y {
                    info!("block collision in {:?} {:?}", block, shape_block);
                    if let Some(p) = new_position_to_send.relative_position.as_mut() {
                        p.x = 0;
                    }
                    new_position_to_send.rotation = None;
                }

                // Check collision with blocks when shape comes from the top
                if block.y == next_y && shape_block.y > block.y && block.x == next_x {
                    if let Some(p) = new_position_to_send.relative_position.as_mut() {
                        p.x = 0;
                        p.y = 0;
                    }
                    new_position_to_send.rotation = None;
                    info!("Block collision from top in {:?} {:?}", block, shape_block);
                    ev_block_collision.send(BlockCollisionEvent);
                    continue 'event_loop;
                }
            }
        }
        // Send an event to notify that the shape position has changed
        if new_position_to_send.relative_position.is_some()
            || new_position_to_send.rotation.is_some()
        {
            debug!("COUCOU");
            new_tetromino_position_ev.send(new_position_to_send);
        }
    }
}
