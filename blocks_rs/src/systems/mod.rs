//! Game systems module
//!
//! This module will contain all ECS systems once the code is refactored.
//! Currently, systems are still in their respective plugin files (board.rs,
//! config.rs, tetroid.rs).
//!
//! ## Future Structure
//!
//! - `board.rs` - Board setup and update systems
//! - `config.rs` - Configuration initialization system
//! - `tetroid.rs` - Tetroid spawning and management systems
//!
//! ## Migration Plan
//!
//! Systems will be moved here during the next refactoring phase while
//! maintaining backward compatibility through re-exports.
