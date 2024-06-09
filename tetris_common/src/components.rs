use bevy::prelude::{Component, Entity};

const DEFAULT_BOARD_WIDTH: usize = 10;
const DEFAULT_BOARD_HEIGHT: usize = 20;

#[derive(Component)]
pub struct Board {
    pub width: usize,
    pub blocks: Vec<Option<Entity>>,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            width,
            blocks: vec![None; height * width],
        }
    }
    pub fn height(&self) -> usize {
        self.blocks.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn insert_entity(&mut self, entity: Entity, x: usize, y: usize) {
        let position = x + y * self.width;

        self.blocks[position] = Some(entity);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        let position = x + y * self.width;
        self.blocks[position]
    }

    pub fn get_all(&self, x: usize, y: usize) -> Vec<Entity> {
        self.blocks.iter().filter_map(|v| v.clone()).collect()
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(DEFAULT_BOARD_HEIGHT, DEFAULT_BOARD_WIDTH)
    }
}

#[derive(Component)]
pub struct CurrentBoard;

#[derive(Component)]
pub struct Shape;

#[derive(Component)]
pub struct ControlledShape;

#[derive(Component)]
pub struct Block;
