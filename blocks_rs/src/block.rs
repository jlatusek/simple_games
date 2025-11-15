use crate::config::Configuration;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn get_vec(&self, config: &Res<Configuration>) -> [usize; 2] {
        [self.x, self.y]
    }

    pub fn get_global(&self, config: &Res<Configuration>) -> Vec2 {
        Vec2::new(
            self.x as f32 * config.block.center_space + config.block.center_space / 2.0
                - config.window.width / 2.0,
            config.window.height / 2.0
                - self.y as f32 * config.block.center_space
                - config.block.center_space / 2.0,
        )
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Position {}

#[derive(Component)]
pub struct BoardBlock {}

#[derive(Component)]
pub struct TetroidBlock {}

impl Default for TetroidBlock {
    fn default() -> Self {
        Self { ..default() }
    }
}

#[derive(Component)]
pub struct MovingBlock {}
