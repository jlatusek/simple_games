//! Game resources module
//!
//! This module contains all ECS resources used in the game.

mod board;
mod config;
mod sprites;

// Re-export all resources
pub use board::Board;
pub use config::Configuration;
pub use sprites::{BlockSprite, GameSprites};
