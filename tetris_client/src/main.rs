use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const BOARD_WIDTH:usize = 10;
const BOARD_HEIGHT:usize = 20;
static BLOCK_SIZE:f32 = 50.;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, renderBlocks).chain())
        // .add_systems(Update, renderBlocks)
        .run()
}


#[derive(Clone)]
struct MyColor(String);

#[derive(Component)]
struct Board{
    blocks: Vec<Option<MyColor>>,
}

#[derive(Component)]
struct Shape {

}

fn setup_game(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());

    let mut blocks = vec![None; BOARD_WIDTH * BOARD_HEIGHT];

    blocks[0] = Some(MyColor("FF0000".into()));
    blocks[1] = Some(MyColor("00FF00".into()));
    blocks[2] = Some(MyColor("0000FF".into()));
    blocks[3] = Some(MyColor("FFFF00".into()));
    blocks[4] = Some(MyColor("FF00FF".into()));

    let mut board = Board {
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
        if let Some(MyColor(color)) = color {
            let x = (index % BOARD_WIDTH) as f32;
            let y = (index / BOARD_HEIGHT) as f32;
            println!("{} {} {}", index, y, BOARD_HEIGHT);
            let shape = Mesh2dHandle(meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE)));
            let color = Color::hex(color);
            if color.is_err() {
                continue
            }

            commands.spawn(MaterialMesh2dBundle{
                mesh: shape,
                material: materials.add(color.unwrap()),
                transform: Transform {
                    translation: Vec3::new(x*BLOCK_SIZE, -y * BLOCK_SIZE, 0.),
                    scale: Vec3::new(1., 1., 1.),
                    ..default()
                },
                ..default()
            });
        }
    }
}
