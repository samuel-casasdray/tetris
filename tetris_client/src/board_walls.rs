use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Color, Component, default, Sprite, SpriteBundle, Transform};

#[derive(Bundle)]
pub struct BoardWallsBundle {
    pub board_wall: BoardWall,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub enum BoardWall {
    Top,
    Bottom,
    Left,
    Right,
}


impl BoardWallsBundle {
    pub fn new(
        pos: Vec2,
        size: Vec2,
        wall_id: BoardWall,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            pos,
            size,
        );

        Self {
            board_wall: wall_id,
            sprite_bundle,
        }
    }
}

fn get_wall_sprite_bundle(
    position: Vec2,
    size: Vec2,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(size),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        transform: Transform {
            translation: position.extend(0.),
            scale: Vec3::splat(1.),
            ..default()
        },
        ..default()
    }
}
