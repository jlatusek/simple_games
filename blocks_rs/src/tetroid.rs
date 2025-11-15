use crate::block::MovingBlock;
use crate::board;
use bevy::prelude::*;

pub struct TetroidPlugin;

impl Plugin for TetroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_tetroid);
    }
}

fn setup_tetroid(
    mut commands: Commands,
    board: Res<board::Board>,
    mut query: Query<&mut Visibility>,
) {
    let entity = board.matrix[2][2].unwrap();
    let mut visibility = query.get_mut(entity).unwrap();
    *visibility = Visibility::Visible;
    commands.entity(entity).insert(MovingBlock {});
}
