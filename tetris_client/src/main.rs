mod block;
mod game_settings;
mod board_walls;
mod pos;

use bevy::prelude::*;
use bevy::sprite::{Mesh2dHandle};
use bevy::window::WindowResized;
use crate::block::{BlockId};
use crate::board_walls::BoardWallsBundle;
use crate::game_settings::GameSettings;
use crate::pos::Pos;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, setup_walls).chain())
        .add_systems(Update, on_resize_system)
        .run()
}

fn on_resize_system(
    mut camera2d_bundle: Query<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
    mut game_settings: ResMut<GameSettings>,
    mut block_mesh_handle: Query<&mut Transform, (With<BlockId>, Without<Camera>)>,
) {
    for e in resize_reader.read() {
        camera2d_bundle.single_mut().translation = Vec3::new(e.width / 2., e.height / -2., 0.);

        let old_block_size = game_settings.block_size;
        let new_block_size = get_block_size(e.width, e.height, game_settings.board_size);
        game_settings.block_size = new_block_size;

        for mut transform in block_mesh_handle.iter_mut(){
            let old_block_scale = transform.scale.x;
            let new_scale = old_block_scale * new_block_size / old_block_size;
            *transform = transform.with_scale(Vec3::new(new_scale, new_scale, new_scale));
        }
    }
}

fn get_block_size(window_width: f32, windows_height: f32, (board_width, board_height): (f32, f32)) -> f32 {
    let width = window_width * 0.4;
    let height = windows_height * 0.9;
    if width / board_width > height / board_height {
        height / board_height
    } else {
        width / board_height
    }
}

fn setup_game(
    mut commands: Commands,
    window: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut game_settings = GameSettings::default();


    let window = window.single();
    let block_size = get_block_size(window.width(), window.height(), game_settings.board_size);
    game_settings.block_size = block_size;
    let (board_width, board_height) = game_settings.board_size;
    commands.insert_resource(game_settings);

    let cam_position = board_to_screen(board_width, board_height, None, block_size);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(cam_position.0 / 2., cam_position.1 / 2., 0.0),
        ..default()
    });

    let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(1.,1.)));
    let material_handle1 = materials.add(Color::hex("FF0000").unwrap());
    let material_handle2 = materials.add(Color::hex("00FF00").unwrap());
    let material_handle3 = materials.add(Color::hex("0000FF").unwrap());

    let block1 = block::BlockBundle::new(
        Pos(0., 0.),
        mesh_handle.clone(),
        material_handle1,
        block_size,
    );
    let block2 = block::BlockBundle::new(
        Pos(1., 0.),
        mesh_handle.clone(),
        material_handle2,
        block_size,
    );
    let block3 = block::BlockBundle::new(
        Pos(3., 0.),
        mesh_handle.clone(),
        material_handle3,
        block_size,
    );

    // Stock le mesh pour les blocks
    commands.spawn(mesh_handle);

    commands.spawn(block1);
    commands.spawn(block2);
    commands.spawn(block3);
}

fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_settings: Res<GameSettings>,
) {
    let block_size = game_settings.block_size;
    let (board_width, board_height) = game_settings.board_size;

    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(1., 1.)));

    let color = materials.add(Color::hex("FFFFFF").unwrap());

    let left_wall = BoardWallsBundle::left_wall(
        Pos(0., 0.),
        mesh.clone(),
        color.clone(),
        block_size,
        board_height,
    );
    let right_wall = BoardWallsBundle::right_wall(
        Pos(game_settings.board_size.0, 0.),
        mesh.clone(),
        color.clone(),
        block_size,
        board_height,
    );
    let top_wall = BoardWallsBundle::top_wall(
        Pos(game_settings.board_size.0, 0.),
        mesh.clone(),
        color.clone(),
        block_size,
        board_width,
    );
    let bottom_wall = BoardWallsBundle::bottom_wall(
        Pos(game_settings.board_size.0, 0.),
        mesh.clone(),
        color.clone(),
        block_size,
        board_width,
    );

    commands.spawn(left_wall);
    commands.spawn(right_wall);
    commands.spawn(top_wall);
    commands.spawn(bottom_wall);
}


struct Offset(f32, f32);

// Ne marche pas encore comme je le souhaite
fn board_to_screen(x: f32, y: f32, offset: Option<Offset>, block_size: f32) -> (f32, f32) {
    match offset {
        None => (x * block_size, y * block_size),
        Some(Offset(x_offset, y_offset)) => (x * block_size + x_offset, y * block_size + y_offset),
    }
}