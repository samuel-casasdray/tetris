use std::cmp::min;

use bevy::hierarchy::Children;
use bevy::log::info;
use bevy::prelude::{
    BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, SpatialBundle, Time,
    With, Without,
};

use crate::board::systems::get_grid_position;
use crate::bundles::{OwnedRelativeBlockBundle, OwnedTetrominoBundle};
use crate::components::{
    Block, Board, DEFAULT_BOARD_HEIGHT, GravityTimer, GridPosition, Owned, RelativeGridPosition,
    Tetromino, TetrominoRotateTo, TetrominoShadow, TetrominoSpeed,
};
use crate::tetromino::events::{BlockCollisionEvent, MovementEvent};

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
                    OwnedRelativeBlockBundle::new(relative_positions, color),
                    SpatialBundle::default(),
                ));
            }
        });
}

pub fn tetromino_next_move_validator(
    mut tetromino_speed_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
    mut controlled_shape_position_q: Query<(&mut GridPosition, &mut Tetromino), With<Owned>>,
    mut controlled_shape_block: Query<&mut RelativeGridPosition, (With<Owned>, With<Block>)>,
) {
    let mut next_move = tetromino_speed_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    let (mut tetromino_position, mut tetromino) = controlled_shape_position_q.single_mut();
    tetromino_position.x += next_move.x;
    tetromino_position.y += next_move.y;

    if let Some(rotation) = tetromino_rotation.0 {
        tetromino.rotation = rotation;
        let blocks = tetromino.shape.get_blocks(rotation);
        for (index, mut tetromino_block_position) in controlled_shape_block.iter_mut().enumerate() {
            *tetromino_block_position = blocks[index].clone();
        }
    }

    *next_move = TetrominoSpeed { x: 0, y: 0 };
    *tetromino_rotation = TetrominoRotateTo(None);
}

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
            commands
                .entity(entity)
                .remove::<TetrominoShadow>()
                .despawn();
        }
    }
}
pub fn shadow_movement(
    tetromino: Query<(&Tetromino, &GridPosition), With<Owned>>,
    mut tetromino_shadow: Query<(&mut TetrominoShadow, &mut RelativeGridPosition), With<Owned>>,
    board_blocks: Query<&GridPosition, (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
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
    mut movement_event: EventReader<MovementEvent>,
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino: Query<&mut Tetromino, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
) {
    let mut next_move = next_move_q.single_mut();

    for movement in movement_event.read() {
        match movement {
            MovementEvent::Right => {
                next_move.x += 1;
            }
            MovementEvent::Left => {
                next_move.x -= 1;
            }
            MovementEvent::Down => {
                next_move.y -= 1;
            }
            MovementEvent::RotationRight => {
                let mut rotation = tetromino_rotation.single_mut();
                let tetromino = tetromino.single_mut();
                rotation.0 = Some(tetromino.get_rotation_right());
            }
            MovementEvent::RotationLeft => {
                let mut rotation = tetromino_rotation.single_mut();
                let tetromino = tetromino.single_mut();
                rotation.0 = Some(tetromino.get_rotation_left());
            }
        }
    }
}

pub fn tetromino_gravity_system(
    time: Res<Time>,
    mut gravity_timer_q: Query<&mut GravityTimer>,
    mut next_move_q: Query<&mut TetrominoSpeed, With<Owned>>,
    board_q: Query<&Board, With<Owned>>,
) {
    if !board_q.single().enable_gravity {
        return;
    }

    let mut gravity_timer = gravity_timer_q.single_mut();
    gravity_timer.timer.tick(time.delta());

    if gravity_timer.timer.finished() {
        let mut next_move = next_move_q.single_mut();
        next_move.y -= 1;
    }
}

pub fn collision_resolver(
    board_q: Query<&Board, With<Owned>>,
    tetromino_q: Query<(&Tetromino, &GridPosition, &Children), With<Owned>>,
    board_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
    shape_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, With<RelativeGridPosition>)>,
    mut tetromino_speed_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotateTo, With<Owned>>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
) {
    let (tetromino, tetromino_position, tetromino_children) = match tetromino_q.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    let mut tetromino_speed = tetromino_speed_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    // Get the tetromino's blocks with the rotation applied if any
    let tetromino_blocks: Vec<GridPosition> = if let Some(rotation) = tetromino_rotation.0 {
        let relative_blocks = tetromino.shape.get_blocks(rotation);
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
        let next_x = shape_block.x + tetromino_speed.x;
        let next_y = shape_block.y + tetromino_speed.y;

        // Check collision with bottom wall
        if next_y < 0 {
            tetromino_speed.y = 0;
            tetromino_rotation.0 = None;
            info!("Bottom wall collision");
            ev_block_collision.send(BlockCollisionEvent);
            return;
        }

        // Check collision with left/right wall
        if next_x >= board.width as i32 || next_x < 0 {
            info!("Left/Right wall collision");
            tetromino_speed.x = 0;
            tetromino_rotation.0 = None;
        }
    }

    for block in board_blocks_q.iter() {
        for shape_block in tetromino_blocks.iter() {
            let next_x = shape_block.x + tetromino_speed.x;
            let next_y = shape_block.y + tetromino_speed.y;

            // Check collision with blocks
            if block.x == next_x && block.y == next_y {
                info!("block collision in {:?} {:?}", block, shape_block);
                tetromino_speed.x = 0;
                tetromino_rotation.0 = None;
            }

            // Check collision with blocks when shape comes from the top
            if block.y == next_y && shape_block.y > block.y && block.x == next_x {
                tetromino_speed.x = 0;
                tetromino_speed.y = 0;
                tetromino_rotation.0 = None;
                info!("Block collision from top in {:?} {:?}", block, shape_block);
                ev_block_collision.send(BlockCollisionEvent);
                return;
            }
        }
    }
}
