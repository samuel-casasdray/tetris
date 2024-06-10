use bevy::prelude::{App, Plugin, Startup, Update};

use crate::events::{BlockCollisionEvent, WallCollisionEvent};
use crate::systems::{collision_check, setup_board};

pub mod components;
pub mod events;
mod shapes;
pub mod systems;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, collision_check);
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy::prelude::{
        BuildChildren, Commands, Entity, IntoSystemConfigs, Query, SpatialBundle, With,
    };
    use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

    use crate::components::{Block, Board, GridPosition, Owned, Tetromino};

    #[test]
    fn terminal_test() {
        App::new()
            .add_systems(Startup, (setup_block, setup_tetromino, draw_system).chain())
            .run()
    }

    fn setup_tetromino(mut commands: Commands) {
        let tetromino = Tetromino::get_random_shape();
        let positions = tetromino.get_blocks_positions().map(|mut position| {
            position.x += 5;
            position.y += 10;
            position
        });
        commands
            .spawn((Owned, tetromino, GridPosition { x: 5, y: 10 }))
            .with_children(|child| {
                for position in positions {
                    child.spawn((Owned, Block, position));
                }
            });
    }

    fn setup_block(mut commands: Commands) {
        let board_entity = commands
            .spawn((Owned, Board::default(), SpatialBundle::default()))
            .id();

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
