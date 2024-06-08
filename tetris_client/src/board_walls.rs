use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Color, default, Sprite, SpriteBundle, Transform};
use crate::pos::Pos;

#[derive(Bundle)]
pub struct BoardWallsBundle {
    position: Pos,
    pub sprite_bundle: SpriteBundle,
}


impl BoardWallsBundle {
    pub fn bottom_wall(
        pos: Pos,
        block_size: f32,
        wall_size: f32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            Vec2 { x: pos.0 * block_size, y: pos.1 * block_size },
            Vec2 { x: wall_size * block_size, y: wall_width },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }
    pub fn top_wall(
        pos: Pos,
        block_size: f32,
        wall_size: f32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            Vec2 { x: pos.0 * block_size, y: pos.1 * block_size },
            Vec2 { x: wall_size * block_size, y: wall_width },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }

    pub fn left_wall(
        pos: Pos,
        block_size: f32,
        wall_size: f32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            Vec2 { x: pos.0 * block_size, y: pos.1 * block_size },
            Vec2 { x: wall_width, y: wall_size * block_size },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }

    pub fn right_wall(
        pos: Pos,
        block_size: f32,
        wall_size: f32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            Vec2 { x: pos.0 * block_size, y: pos.1 * block_size },
            Vec2 { x: wall_width, y: wall_size * block_size },
        );

        Self {
            position: pos,
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
