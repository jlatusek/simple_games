use crate::config::Configuration;
use bevy::prelude::*;

#[derive(Clone)]
pub enum BlockType {
    Tetroid,
    Board,
}

#[derive(Component, Clone)]
pub struct BaseBlock {
    pub position: Vec2,
    pub entity: Option<Entity>,
}

impl Default for BaseBlock {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            entity: None,
        }
    }
}

pub trait Block {
    fn btype(&self) -> BlockType;
}

impl BaseBlock {
    pub fn board_cord_to_global(&self, config: &Res<Configuration>) -> Vec2 {
        {
            Vec2::new(
                self.position.x * config.block.center_space + config.block.center_space / 2.0
                    - config.window.width / 2.0,
                config.window.height / 2.0
                    - self.position.y * config.block.center_space
                    - config.block.center_space / 2.0,
            )
        }
    }
}

#[derive(Component)]
pub struct BoardBlock {}

impl Block for BoardBlock {
    fn btype(&self) -> BlockType {
        BlockType::Board
    }
}

#[derive(Component)]
pub struct TetroidBlock {}

impl Default for TetroidBlock {
    fn default() -> Self {
        Self { ..default() }
    }
}
impl Block for TetroidBlock {
    fn btype(&self) -> BlockType {
        BlockType::Tetroid
    }
}
