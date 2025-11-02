use bevy::prelude::*;

#[derive(Component)]
pub struct BaseBlock {
    pub x: usize,
    pub y: usize,
}

impl Default for BaseBlock {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
