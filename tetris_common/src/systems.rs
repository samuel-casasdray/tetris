use bevy::prelude::{Children, Commands, EventWriter, Query, With};

use crate::components::{Block, Board, ControlledShape, CurrentBoard, Shape};
use crate::events::{BlockCollisionEvent, WallCollisionEvent};

pub fn setup_board(mut command: Commands) {
    command.spawn((CurrentBoard, Board::default()));
}

pub fn collision_check(
    current_board: Query<(&Board, &Children), With<CurrentBoard>>,
    controlled_shape: Query<(&Shape, &Children), With<ControlledShape>>,
    blocks: Query<&Block>,
    mut ev_block_collision: EventWriter<BlockCollisionEvent>,
    mut ev_wall_collision: EventWriter<WallCollisionEvent>,
) {
    let (_, controlled_shape_entities) = match controlled_shape.get_single() {
        Ok(query) => query,
        Err(_) => return,
    };

    let (_board, children) = current_board.single();
    for block in blocks.iter_many(children) {
        for shape_blocks in blocks.iter_many(controlled_shape_entities) {
            // Check collision with shapes block
            if block.x == shape_blocks.x && block.y == shape_blocks.y {
                ev_block_collision.send(BlockCollisionEvent);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy::prelude::{BuildChildren, Commands, EventReader, IntoSystemConfigs, Res, Resource};

    use crate::components::{Block, Board, ControlledShape, CurrentBoard, Shape};
    use crate::events::{BlockCollisionEvent, WallCollisionEvent};
    use crate::systems::collision_check;

    #[derive(Resource)]
    struct TestState(bool);

    #[allow(private_interfaces)]
    pub fn test_checker_collision(
        test_init_state: Res<TestState>,
        mut ev_block_collision: EventReader<BlockCollisionEvent>,
        mut ev_wall_collision: EventReader<BlockCollisionEvent>,
    ) {
        let mut should_assert = test_init_state.0;
        for _ in ev_block_collision.read() {
            should_assert = !should_assert;
        }
        assert!(should_assert)
    }

    pub fn setup_board_no_collision(mut commands: Commands) {
        commands.spawn((CurrentBoard, Board::default()))
            .with_children(|parent| {
                parent.spawn(Block {
                    x: 10,
                    y: 10,
                });
            });

        commands.spawn((ControlledShape, Shape)).with_children(|parent| {
            parent.spawn(Block {
                x: 11,
                y: 10,
            });
        });
        commands.insert_resource(TestState(true))
    }

    pub fn setup_board_collision(mut commands: Commands) {
        commands.spawn((CurrentBoard, Board::default()))
            .with_children(|parent| {
                parent.spawn(Block {
                    x: 10,
                    y: 10,
                });
            });

        commands.spawn((ControlledShape, Shape)).with_children(|parent| {
            parent.spawn(Block {
                x: 11,
                y: 10,
            });
        });
        commands.insert_resource(TestState(true))
    }

    #[test]
    fn no_collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(Startup, (setup_board_no_collision, collision_check, test_checker_collision).chain())
            .run()
    }

    #[test]
    fn collision_should_occur() {
        App::new()
            .add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(Startup, (setup_board_collision, collision_check, test_checker_collision).chain())
            .run()
    }
}