use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Color, default, Sprite, SpriteBundle, Transform};

use crate::board::{BoardCalculator, BoardPoint};

#[derive(Bundle)]
pub struct BoardWallsBundle {
    position: BoardPoint,
    pub sprite_bundle: SpriteBundle,
}


impl BoardWallsBundle {
    pub fn bottom_wall(
        pos: BoardPoint,
        board_calculator: &BoardCalculator,
        wall_size: i32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            board_calculator.window_relative_position(&pos),
            Vec2 { x: board_calculator.window_relative_size(wall_size), y: wall_width },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }
    pub fn top_wall(
        pos: BoardPoint,
        board_calculator: &BoardCalculator,
        wall_size: i32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            board_calculator.window_relative_position(&pos),
            Vec2 { x: board_calculator.window_relative_size(wall_size), y: wall_width },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }

    pub fn left_wall(
        pos: BoardPoint,
        board_calculator: &BoardCalculator,
        wall_size: i32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            board_calculator.window_relative_position(&pos),
            Vec2 { x: wall_width, y: board_calculator.window_relative_size(wall_size) },
        );

        Self {
            position: pos,
            sprite_bundle,
        }
    }

    pub fn right_wall(
        pos: BoardPoint,
        board_calculator: &BoardCalculator,
        wall_size: i32,
        wall_width: f32,
    ) -> Self {
        let sprite_bundle = get_wall_sprite_bundle(
            board_calculator.window_relative_position(&pos),
            Vec2 { x: wall_width, y: board_calculator.window_relative_size(wall_size) },
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
