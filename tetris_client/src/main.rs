use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const BOARD_WIDTH:usize = 10;
const BOARD_HEIGHT:usize = 20;
const BLOCK_SIZE:f32 = 50.;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, renderBlocks)
        .run()
}


#[derive(Clone)]
struct MyColor(u32);

#[derive(Component)]
struct Board{
    blocks: Vec<Option<MyColor>>,
}

fn setup_game(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());

    let mut blocks = vec![None; BOARD_WIDTH * BOARD_HEIGHT];

    blocks.push(Some(MyColor(0xFFFF0000)));
    blocks.push(Some(MyColor(0xFFFF0000)));
    blocks.push(Some(MyColor(0xFFFF0000)));
    blocks.push(Some(MyColor(0xFFFF0000)));
    blocks.push(Some(MyColor(0xFFFF0000)));

    let mut board = Board{
        blocks,
    };

    commands.spawn(board);
}

fn renderBlocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Query<&Board>,
){
    for (index, color) in board.single().blocks.iter().enumerate(){
        let x = index as f32;
        let y = (index % BOARD_WIDTH) as f32;
        if let Some(MyColor(color)) = color {
            let shape = Mesh2dHandle(meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE)));
            let color = Color::hex(color);

            commands.spawn(MaterialMesh2dBundle{
                mesh: shape,
                material: materials.add(color),
                    ..default()
            });
        }
    }
}
