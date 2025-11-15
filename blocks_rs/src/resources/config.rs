use crate::constants::*;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
}

impl From<&Window> for WindowConfig {
    fn from(w: &Window) -> Self {
        Self {
            width: w.width(),
            height: w.height(),
            title: w.title.clone(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            title: DEFAULT_WINDOW_TITLE.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockConfig {
    pub size: f32,
    pub center_space: f32,
    pub move_delay: f32,
}

impl Default for BlockConfig {
    fn default() -> Self {
        Self {
            size: BLOCK_SIZE,
            center_space: BLOCK_CENTER_SPACE,
            move_delay: BLOCK_MOVE_DELAY,
        }
    }
}

#[derive(Resource, Default)]
pub struct Configuration {
    pub window: WindowConfig,
    pub block: BlockConfig,
}
