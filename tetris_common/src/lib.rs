use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Startup};

use crate::events::BlockCollisionEvent;
use crate::systems::{
    collision_resolver, relative_position_system, setup_board, tetromino_blocks_fixer,
    tetromino_gravity_system, tetromino_next_move_validator, tetromino_spawner,
};

mod bundles;
pub mod components;
pub mod events;
mod shapes;
pub mod systems;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockCollisionEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(
                PreUpdate,
                (
                    (tetromino_spawner, relative_position_system).chain(),
                    (
                        tetromino_gravity_system,
                        collision_resolver,
                        tetromino_next_move_validator,
                        tetromino_blocks_fixer,
                    )
                        .chain(),
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::app::{App, Startup};
    use bevy::prelude::{
        BuildChildren, Commands, Entity, IntoSystemConfigs, Query, SpatialBundle, Timer, TimerMode,
        Update, With,
    };
    use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

    use crate::components::{Block, Board, GravityTimer, GridPosition, Owned, Tetromino};
    use crate::systems::{relative_position_system, tetromino_gravity_system};

    #[test]
    fn terminal_test() {
        let mut app = App::new();

        app.add_systems(Startup, (setup_block, setup_tetromino).chain())
            .add_systems(
                Update,
                (
                    relative_position_system,
                    tetromino_gravity_system,
                    draw_system,
                )
                    .chain(),
            );

        app.update();
        app.update();
        app.update();
        app.update();
        app.update();
        app.update();
        app.update();
        app.update();
    }

    fn setup_tetromino(mut commands: Commands) {
        let tetromino = Tetromino::get_random_shape();
        let positions = tetromino.get_blocks_positions();
        commands
            .spawn((Owned, tetromino, GridPosition { x: 5, y: 10 }))
            .with_children(|child| {
                for relative_positions in positions {
                    child.spawn((
                        Owned,
                        Block,
                        relative_positions,
                        GridPosition { x: 0, y: 0 },
                    ));
                }
            });
    }

    fn setup_block(mut commands: Commands) {
        let board_entity = commands
            .spawn((Owned, Board::default(), SpatialBundle::default()))
            .id();

        commands.spawn(GravityTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        });

        let block_entities: Vec<Entity> = (0usize..10usize)
            .flat_map(|x| (0usize..5usize).map(move |y| (x, y)))
            .map(|(x, y)| {
                commands
                    .spawn((
                        Owned,
                        Block,
                        GridPosition {
                            x: x as i32,
                            y: y as i32,
                        },
                        SpatialBundle::default(),
                    ))
                    .id()
            })
            .collect();

        commands.entity(board_entity).push_children(&block_entities);
    }

    fn draw_system(
        blocks: Query<&GridPosition, (With<Owned>, With<Block>)>,
        board: Query<&Board, With<Owned>>,
    ) {
        let board = board.single();
        let mut game_board = vec![" "; board.height * board.width];

        for block in blocks.iter() {
            let x = block.x;
            // we need to draw from top to bottom
            let y = (board.height - 1) as i32 - block.y;

            game_board[x as usize + (y * board.width as i32) as usize] = "x";
        }

        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        });

        for s in game_board {
            grid.add(Cell::from(s));
        }

        let separator = "-".repeat(board.width * 2);
        println!("{}", separator);
        println!("{}", grid.fit_into_columns(board.width));
        println!("{}", separator);
    }
}
