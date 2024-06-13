use bevy::prelude::*;
use bevy::window::WindowResized;

use tetris_common::CommonPlugin;
use tetris_common::components::{Block, GridPosition};
use tetris_common::events::MovementEvent;

use crate::board_ui_calculator::{
    BoardUICalculator, DEFAULT_BOARD_HEIGHT, DEFAULT_BOARD_WIDTH, get_window_position,
    MAX_BOARD_HEIGHT_PERCENT, MAX_BOARD_WIDTH_PERCENT,
};
use crate::board_walls::{BoardWall, BoardWallsBundle};

mod block;
mod board_ui_calculator;
mod board_walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_systems(Startup, (setup_resources, setup_game, setup_walls).chain())
        .add_systems(
            Update,
            (
                add_missing_sprite_to_block,
                on_resize_system,
                update_sprite_position,
                keyboard_iter,
            ),
        )
        .run()
}

fn keyboard_iter(keys: Res<ButtonInput<KeyCode>>, mut movement_event: EventWriter<MovementEvent>) {
    if keys.just_pressed(KeyCode::ArrowRight) {
        movement_event.send(MovementEvent::Right);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        movement_event.send(MovementEvent::Left);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        movement_event.send(MovementEvent::Down);
    }
}

fn add_missing_sprite_to_block(
    mut commands: Commands,
    blocks: Query<(Entity, &GridPosition), (With<Block>, Without<Sprite>)>,
    board_calculator: Res<BoardUICalculator>,
) {
    let colors = [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW];

    for (entity, block) in blocks.iter() {
        let x = block.x;
        let y = block.y;
        commands.entity(entity).insert(block::bloc_sprite(
            &board_calculator,
            GridPosition { x, y },
            colors[(x + y) as usize % colors.len()],
            board_calculator.block_size,
        ));
    }
}

fn update_sprite_position(
    mut blocks: Query<(&mut Transform, &GridPosition), With<Block>>,
    board_calculator: Res<BoardUICalculator>,
) {
    for (mut transform, grid_position) in blocks.iter_mut() {
        transform.translation = board_calculator
            .window_relative_position(grid_position)
            .extend(0.)
    }
}

fn on_resize_system(
    mut camera2d_bundle: Query<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
    mut board_calculator: ResMut<BoardUICalculator>,
    mut blocks: Query<(&mut Transform, &GridPosition), (With<Block>, Without<Camera>)>,
    mut walls: Query<(&mut Transform, &BoardWall), (Without<Camera>, Without<Block>)>,
) {
    for window in resize_reader.read() {
        camera2d_bundle.single_mut().translation =
            Vec3::new(window.width / 2., window.height / 2., 0.);
        let old_block_size = board_calculator.block_size;
        board_calculator.set_window_position(window.width, window.height);
        board_calculator.block_size = get_block_size(
            window.width,
            window.height,
            board_calculator.board_width,
            board_calculator.board_height,
        );
        for mut block in blocks.iter_mut() {
            let new_scale = block.0.scale.x * board_calculator.block_size / old_block_size;
            *block.0 = block
                .0
                .with_scale(Vec3::new(new_scale, new_scale, new_scale))
                .with_translation(
                    board_calculator
                        .window_relative_position(&GridPosition {
                            x: block.1.x,
                            y: block.1.y,
                        })
                        .extend(0.),
                )
        }
        let walls_size = board_calculator.window_relative_board_walls();
        for mut wall in walls.iter_mut().zip(walls_size) {
            *wall.0 .0 = wall
                .0
                 .0
                .with_scale(Vec3::new(
                    wall.0 .0.scale.x * board_calculator.block_size / old_block_size,
                    wall.0 .0.scale.y * board_calculator.block_size / old_block_size,
                    1.,
                ))
                .with_translation(wall.1 .0.extend(0.))
        }
    }
}

fn get_block_size(width: f32, height: f32, board_width: usize, board_height: usize) -> f32 {
    let width = width * MAX_BOARD_WIDTH_PERCENT;
    let height = height * MAX_BOARD_HEIGHT_PERCENT;
    (width / board_width as f32).min(height / board_height as f32)
}

fn setup_resources(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let block_size = get_block_size(
        window.width(),
        window.height(),
        DEFAULT_BOARD_WIDTH,
        DEFAULT_BOARD_HEIGHT,
    );
    let board_calculator = BoardUICalculator::new(
        get_window_position(block_size, window.width(), window.height()),
        block_size,
        DEFAULT_BOARD_HEIGHT,
        DEFAULT_BOARD_WIDTH,
    );

    commands.insert_resource(board_calculator);
}

fn setup_game(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

fn setup_walls(mut commands: Commands, board_cal: Res<BoardUICalculator>) {
    let walls = board_cal.window_relative_board_walls();
    let walls_order = [
        BoardWall::Top,
        BoardWall::Bottom,
        BoardWall::Left,
        BoardWall::Right,
    ];

    for wall in walls.iter().zip(walls_order) {
        let wall_bundle = BoardWallsBundle::new(wall.0 .0, wall.0 .1, wall.1);
        commands.spawn(wall_bundle);
    }
}
