mod block;
mod game_settings;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::WindowResized;
use crate::block::{BlockId};
use crate::game_settings::GameSettings;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, render_board).chain())
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

        // TODO: je sais pas si c'est nécessaire de mettre à jour le mesh ou s'il prend automatiquement la taille du transform.
        for mut transform in block_mesh_handle.iter_mut(){
            let old_block_scale = transform.scale.clone().x;
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

    let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(block_size, block_size)));
    let material_handle1 = materials.add(Color::hex("FF0000").unwrap());
    let material_handle2 = materials.add(Color::hex("00FF00").unwrap());
    let material_handle3 = materials.add(Color::hex("0000FF").unwrap());

    let block1 = block::BlockBundle::new(
        block::Pos(0., 0.),
        mesh_handle.clone(),
        material_handle1,
        block_size,
    );
    let block2 = block::BlockBundle::new(
        block::Pos(1., 0.),
        mesh_handle.clone(),
        material_handle2,
        block_size,
    );
    let block3 = block::BlockBundle::new(
        block::Pos(3., 0.),
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

fn render_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_settings: Res<GameSettings>,
) {
    let block_size = game_settings.block_size;
    let (board_width, board_height) = game_settings.board_size;
    let wall_width = 5.;

    let horizontal_bar_width = block_size * board_width;
    let vertical_bar_height = block_size * board_height;

    let horizontal_bar = Mesh2dHandle(meshes.add(Rectangle::new(horizontal_bar_width + block_size, wall_width)));
    let vertical_bar = Mesh2dHandle(meshes.add(Rectangle::new(wall_width, vertical_bar_height + block_size)));

    let color = Color::hex("FFFFFF").unwrap();

    let top_position = board_to_screen(0., board_height, Some(Offset(horizontal_bar_width / 2., block_size / 2.)), block_size);
    let bottom_position = board_to_screen(0., 0., Some(Offset(horizontal_bar_width / 2., block_size / -2.)), block_size);
    let left_position = board_to_screen(0., 0., Some(Offset(block_size / -2., vertical_bar_height / 2.)), block_size);
    let right_position = board_to_screen(board_width, 0., Some(Offset(block_size / 2., vertical_bar_height / 2.)), block_size);

    let shapes = [
        (horizontal_bar.clone(), top_position.0, top_position.1),
        (horizontal_bar, bottom_position.0, bottom_position.1),
        (vertical_bar.clone(), left_position.0, left_position.1),
        (vertical_bar, right_position.0, right_position.1),
    ];

    for (shape, x, y) in shapes {
        let lol = MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        };
        commands.spawn(lol);
    }
}


struct Offset(f32, f32);

// Ne marche pas encore comme je le souhaite
fn board_to_screen(x: f32, y: f32, offset: Option<Offset>, block_size: f32) -> (f32, f32) {
    match offset {
        None => (x * block_size, y * block_size),
        Some(Offset(x_offset, y_offset)) => (x * block_size + x_offset, y * block_size + y_offset),
    }
}