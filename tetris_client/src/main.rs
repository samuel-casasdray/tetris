use bevy::prelude::*;
use bevy::window::WindowResized;

use tetris_common::CommonPlugin;
use tetris_common::components::Block;

use crate::board_ui_calculator::{BoardUICalculator, InBoardPoint};
use crate::board_walls::{BoardWall, BoardWallsBundle};

pub const DEFAULT_BOARD_WIDTH: usize = 10;
pub const DEFAULT_BOARD_HEIGHT: usize = 20;

mod block;
mod board_walls;
mod board_ui_calculator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_systems(Startup, (setup_resources, setup_game, setup_walls).chain())
        .add_systems(Update, (setup_sprites, on_resize_system))
        .run()
}

fn setup_sprites(
    mut commands: Commands,
    blocks: Query<(Entity, &Block), Without<Sprite>>,
    board_calculator: Res<BoardUICalculator>,
) {
    let colors = [
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
    ];

    for (entity, block) in blocks.iter() {
        let x = block.x;
        let y = block.y;
        commands.entity(entity).insert(block::new_block(
            &board_calculator,
            InBoardPoint(x, y),
            colors[(x + y) as usize % colors.len()],
            board_calculator.block_size,
        ));
    }
}

fn on_resize_system(
    mut camera2d_bundle: Query<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
    mut board_calculator: ResMut<BoardUICalculator>,
    mut blocks: Query<&mut Transform, (With<Block>, Without<Camera>)>,
) {
    for window in resize_reader.read() {
        camera2d_bundle.single_mut().translation = Vec3::new(window.width / 2., window.height / 2., 0.);
        let old_block_size = board_calculator.block_size;
        board_calculator.block_size = get_block_size(window.width, window.height, board_calculator.board_width, board_calculator.board_height);
        for mut block in blocks.iter_mut() {
            let old_block_scale = block.scale.x;
            let new_scale = old_block_scale * board_calculator.block_size / old_block_size;
            *block = block.with_scale(Vec3::new(new_scale, new_scale, new_scale));
        }
    }
}

fn get_block_size(width: f32, height: f32, board_width: usize, board_height: usize) -> f32 {
    let width = width * 0.4;
    let height = height * 0.9;
    (width / board_width as f32).min(height / board_height as f32)
}

fn setup_resources(
    mut commands: Commands,
    window: Query<&Window>,
) {
    let window = window.single();
    let block_size = get_block_size(window.width(), window.height(), DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_HEIGHT);
    let x = (window.width() - block_size * (DEFAULT_BOARD_WIDTH + 2) as f32) / 2.;
    let y = (window.height() - block_size * DEFAULT_BOARD_HEIGHT as f32) / 2.;
    let board_calculator = BoardUICalculator::new(
        Vec2::new(x, y),
        block_size,
        DEFAULT_BOARD_HEIGHT,
        DEFAULT_BOARD_WIDTH,
    );

    commands.insert_resource(board_calculator);
}

fn setup_game(
    mut commands: Commands,
    window: Query<&Window>,
) {
    let window = window.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

fn setup_walls(
    mut commands: Commands,
    board_cal: Res<BoardUICalculator>,
) {
    let walls = board_cal.window_relative_board_walls();
    let walls_order = [BoardWall::Top, BoardWall::Bottom, BoardWall::Left, BoardWall::Right];

    for wall in walls.iter().zip(walls_order) {
        let wall_bundle = BoardWallsBundle::new(
            wall.0.0,
            wall.0.1,
            wall.1,
        );
        commands.spawn(wall_bundle);
    }
}
