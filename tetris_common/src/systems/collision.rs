use bevy::prelude::{Children, EventWriter, Query, With};

use crate::components::{Block, Board, Fake, GridPosition, Owned, Tetromino};
use crate::events::{BlockCollisionEvent, WallCollisionEvent};

pub fn collision_check(
    current_board_children: Query<&Children, With<Owned>>,
    current_board: Query<&Board, With<Owned>>,
    controlled_shape: Query<(&Tetromino, &Children), (With<Fake>, With<Owned>)>,
    blocks: Query<&GridPosition, With<Block>>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
    mut ev_wall_collision: EventWriter<WallCollisionEvent>,
) {
    let (_, controlled_shape_entities) = match controlled_shape.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    let board = current_board.single();
    for shape_block in blocks.iter_many(controlled_shape_entities) {
        // Check collision with walls
        match (shape_block.x, shape_block.y) {
            (_, y) if y < 0 => {
                ev_wall_collision.send(WallCollisionEvent);
                return;
            }
            (x, _) if x >= board.width as i32 || x < 0 => {
                ev_wall_collision.send(WallCollisionEvent);
                return;
            }
            _ => {}
        };
    }

    let children_result = current_board_children.get_single();
    if let Ok(children) = children_result {
        for block in blocks.iter_many(children) {
            for shape_blocks in blocks.iter_many(controlled_shape_entities) {
                // Check collision with shapes block
                if block.x == shape_blocks.x && block.y == shape_blocks.y {
                    ev_block_collision.send(BlockCollisionEvent);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy::prelude::{BuildChildren, Commands, EventReader, IntoSystemConfigs, Res, Resource};

    use crate::components::{Block, Board, Fake, GridPosition, Owned, Tetromino};
    use crate::events::{BlockCollisionEvent, WallCollisionEvent};
    use crate::systems::collision_check;

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
        commands
            .spawn((Owned, Board::default()))
            .with_children(|parent| {
                parent.spawn((Block, GridPosition { x: 10, y: 10 }));
            });

        commands
            .spawn((Fake, Tetromino::get_random_shape()))
            .with_children(|parent| {
                parent.spawn((Block, GridPosition { x: 11, y: 10 }));
            });
        commands.insert_resource(ShouldCollide(true))
    }

    pub fn setup_board_block_collision(mut commands: Commands) {
        commands
            .spawn((Owned, Board::default()))
            .with_children(|parent| {
                parent.spawn((Block, GridPosition { x: 10, y: 10 }));
            });

        commands
            .spawn((Fake, Tetromino::get_random_shape()))
            .with_children(|parent| {
                parent.spawn((Block, GridPosition { x: 11, y: 10 }));
            });
        commands.insert_resource(ShouldCollide(true))
    }

    pub fn setup_board_wall_collision(mut commands: Commands) {
        commands.spawn((Owned, Board::default()));
        commands
            .spawn((Fake, Tetromino::get_random_shape()))
            .with_children(|parent| {
                parent.spawn((Block, GridPosition { x: -1, y: 10 }));
            });
        commands.insert_resource(ShouldCollide(true))
    }

    #[test]
    fn block_no_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_no_collision,
                    collision_check,
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
            .add_event::<WallCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_block_collision,
                    collision_check,
                    test_checker_collision,
                )
                    .chain(),
            )
            .run()
    }

    #[test]
    fn no_wall_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_no_collision,
                    collision_check,
                    test_checker_collision,
                )
                    .chain(),
            )
            .run()
    }

    #[test]
    fn wall_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(
                Startup,
                (
                    setup_board_wall_collision,
                    collision_check,
                    test_checker_collision,
                )
                    .chain(),
            )
            .run()
    }
}
