use crate::config::Configuration;
use bevy::prelude::*;

#[derive(Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

#[derive(Clone)]
pub enum BlockType {
    Tetroid,
    Board,
}

#[derive(Component, Clone)]
pub struct BaseBlock {
    pub position: Position,
    pub entity: Option<Entity>,
}

impl Default for BaseBlock {
    fn default() -> Self {
        Self {
            position: Position::ZERO,
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
                self.position.x as f32 * config.block.center_space
                    + config.block.center_space / 2.0
                    - config.window.width / 2.0,
                config.window.height / 2.0
                    - self.position.y as f32 * config.block.center_space
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

#[derive(Component)]
pub struct MovingBlock {}

#[derive(Component)]
pub struct StoppedBlock {}
