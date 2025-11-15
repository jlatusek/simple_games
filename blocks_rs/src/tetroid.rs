use crate::block::MovingBlock;
use crate::{board, config};
use bevy::prelude::*;

pub struct TetroidPlugin;

impl Plugin for TetroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_tetroid);
    }
}

fn setup_tetroid(
    mut commands: Commands,
    config: Res<config::Configuration>,
    board: Res<board::Board>,
    mut query: Query<&mut Visibility>,
) {
    let entity = board.matrix[board.width / 2][0].unwrap();
    let mut visibility = query.get_mut(entity).unwrap();
    *visibility = Visibility::Visible;
    commands.entity(entity).insert(MovingBlock {});
}
