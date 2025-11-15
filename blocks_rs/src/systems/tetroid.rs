//! Tetroid spawning and management systems

use crate::components::MovingBlock;
use crate::resources::Board;
use bevy::prelude::*;

/// System to spawn the initial tetroid piece
pub fn setup_tetroid(mut commands: Commands, board: Res<Board>, mut query: Query<&mut Visibility>) {
    // Spawn the first piece at the center top of the board
    let center_x = board.width() / 2;
    let start_y = 0;

    // Safely get the entity at the spawn position
    let Some(entity) = board.get(center_x, start_y) else {
        error!(
            "Failed to get entity at spawn position ({}, {})",
            center_x, start_y
        );
        return;
    };

    // Make the block visible and mark it as moving
    if let Ok(mut visibility) = query.get_mut(entity) {
        *visibility = Visibility::Visible;
        commands.entity(entity).insert(MovingBlock);
    } else {
        error!("Failed to get visibility for spawned tetroid");
    }
}
