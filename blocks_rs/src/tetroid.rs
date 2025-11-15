use crate::components::MovingBlock;
use crate::resources::{Board, Configuration};
use bevy::prelude::*;

pub struct TetroidPlugin;

impl Plugin for TetroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_tetroid);
    }
}

fn setup_tetroid(
    mut commands: Commands,
    config: Res<Configuration>,
    board: Res<Board>,
    mut query: Query<&mut Visibility>,
) {
    let entity = board.get(board.width() / 2, 0).unwrap();
    let mut visibility = query.get_mut(entity).unwrap();
    *visibility = Visibility::Visible;
    commands.entity(entity).insert(MovingBlock {});
}
