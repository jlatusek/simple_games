//! Game systems module
//!
//! This module contains all ECS systems used in the game.

pub mod board;
pub mod config;
pub mod control;
pub mod tetroid;

// Re-export commonly used systems and resources
pub use board::{setup_board, update_board};
pub use config::setup_config;
pub use tetroid::setup_tetroid;
