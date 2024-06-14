use bevy::prelude::{Children, EventWriter, Query, With, Without};

use crate::components::{Block, Board, GridPosition, Owned, RelativeGridPosition, Tetromino, TetrominoRotation, TetrominoSpeed};
use crate::events::BlockCollisionEvent;

pub fn collision_resolver(
    current_board: Query<&Board, With<Owned>>,
    controlled_shape: Query<(&Tetromino, &Children), With<Owned>>,
    board_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, Without<RelativeGridPosition>)>,
    shape_blocks_q: Query<&GridPosition, (With<Block>, With<Owned>, With<RelativeGridPosition>)>,
    mut tetromino_speed_q: Query<&mut TetrominoSpeed, With<Owned>>,
    mut tetromino_rotation: Query<&mut TetrominoRotation, With<Owned>>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
) {
    let (_, controlled_shape_entities) = match controlled_shape.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    let mut speed = tetromino_speed_q.single_mut();
    let mut tetromino_rotation = tetromino_rotation.single_mut();

    let board = current_board.single();
    let mut cancel_rotation = false;
    let mut stop = false;
    for (shape_block, (x, y)) in shape_blocks_q.iter_many(controlled_shape_entities).zip(tetromino_rotation.rotations) {
        // Check collision with walls
        let next_x = shape_block.x + speed.x;
        let next_y = shape_block.y + speed.y;

        if next_y < 0 && !stop {
            speed.y = 0;
            println!("Bottom wall collision");
            ev_block_collision.send(BlockCollisionEvent);
            stop = true;
        }

        if (next_x >= board.width as i32 || next_x < 0) && !stop {
            println!("Left/Right wall collision");
            speed.x = 0;
        }

        let next_x = shape_block.x + x;
        let next_y = shape_block.y + y;

        if next_y < 0 || next_x >= board.width as i32 || next_x < 0 {
            cancel_rotation = true
        }
    }

    for block in board_blocks_q.iter() {
        for (shape_block, (x, y)) in shape_blocks_q.iter_many(controlled_shape_entities).zip(tetromino_rotation.rotations) {
            let next_x = shape_block.x + speed.x;
            let next_y = shape_block.y + speed.y;
            // Check collision with shapes block
            if (block.x == next_x && block.y == next_y) && !stop {
                println!("COLLISION -> {:?} {:?}", block, shape_block);
                speed.x = 0;
            }
            if block.y == next_y && shape_block.y > block.y && block.x == next_x && !stop {
                speed.x = 0;
                speed.y = 0;
                println!("COLLISION -> lock {:?} {:?}", block, shape_block);
                ev_block_collision.send(BlockCollisionEvent);
                stop = true;
            }

            let next_x = shape_block.x + x;
            let next_y = shape_block.y + y;
            // Check collision for the rotation
            if (block.x == next_x && block.y == next_y) || (block.y == next_y && shape_block.y > block.y && block.x == next_x) {
                cancel_rotation = true
            }
        }
    }

    if cancel_rotation {
        *tetromino_rotation = TetrominoRotation::new();
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy::prelude::{
        BuildChildren, Color, Commands, EventReader, IntoSystemConfigs, Res, Resource,
    };

    use crate::bundles::OwnedNextMoveBundle;
    use crate::components::{Block, Board, Fake, GridPosition, Owned, Tetromino};
    use crate::events::BlockCollisionEvent;
    use crate::systems::collision_resolver;

    #[derive(Resource)]
    struct ShouldCollide(bool);

    #[allow(private_interfaces)]
    pub fn test_checker_collision(
        test_init_state: Res<ShouldCollide>,
        mut ev_block_collision: EventReader<BlockCollisionEvent>,
        mut ev_wall_collision: EventReader<BlockCollisionEvent>,
    ) {
        let mut should_assert = test_init_state.0;
        for _ in ev_block_collision.read() {
            should_assert = !should_assert;
        }
        assert!(should_assert);

        should_assert = test_init_state.0;
        for _ in ev_wall_collision.read() {
            should_assert = !should_assert;
        }
        assert!(should_assert);
    }

    pub fn setup_board_no_collision(mut commands: Commands) {
        commands.spawn(OwnedNextMoveBundle::new());
        commands
            .spawn((Owned, Board::default()))
            .with_children(|parent| {
                parent.spawn((Block::new(Color::RED), GridPosition { x: 10, y: 10 }));
            });

        commands
            .spawn((Fake, Tetromino::get_random_shape()))
            .with_children(|parent| {
                parent.spawn((Block::new(Color::RED), GridPosition { x: 11, y: 10 }));
            });
        commands.insert_resource(ShouldCollide(true))
    }

    pub fn setup_board_block_collision(mut commands: Commands) {
        commands.spawn(OwnedNextMoveBundle::new());
        commands
            .spawn((Owned, Board::default()))
            .with_children(|parent| {
                parent.spawn((Block::new(Color::RED), GridPosition { x: 10, y: 10 }));
            });

        commands
            .spawn((Fake, Tetromino::get_random_shape()))
            .with_children(|parent| {
                parent.spawn((Block::new(Color::RED), GridPosition { x: 11, y: 10 }));
            });
        commands.insert_resource(ShouldCollide(true))
    }

    #[test]
    fn block_no_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_no_collision,
                    collision_resolver,
                    test_checker_collision,
                )
                    .chain(),
            )
            .run()
    }

    #[test]
    fn block_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_block_collision,
                    collision_resolver,
                    test_checker_collision,
                )
                    .chain(),
            )
            .run()
    }
}
