use crate::block::{BaseBlock, TetroidBlock};
use crate::board::Board;
use crate::config::Configuration;
use crate::{board, config};
use bevy::prelude::*;

pub struct TetroidPlugin;

#[derive(Resource)]
struct BlocksMoveTimer {
    timer: Timer,
}

impl Plugin for TetroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_tetroid)
            .add_systems(FixedUpdate, update_cube);
    }
}

fn setup_tetroid(
    mut commands: Commands,
    config: Res<config::Configuration>,
    board: Res<board::Board>,
    mut query: Query<(&mut Visibility)>,
) {
    commands.insert_resource(BlocksMoveTimer {
        timer: Timer::from_seconds(config.block.move_delay, TimerMode::Repeating),
    });
    let entity = board.matrix[3][3].unwrap();
    let mut visibility = query.get_mut(entity).unwrap();
    *visibility = Visibility::Visible;
}

fn update_cube(
    mut block_query: Query<(&mut BaseBlock, &mut Visibility), With<TetroidBlock>>,
    board: Res<Board>,
    time: Res<Time>,
    mut blocks_timer: ResMut<BlocksMoveTimer>,
    config: Res<Configuration>,
) {
    blocks_timer.timer.tick(time.delta());

    if blocks_timer.timer.is_finished() {
        let mut new_visible: Vec<Entity> = Vec::new();
        for (mut block, mut visibility) in block_query.iter_mut() {
            if visibility.eq(&Visibility::Visible) {
                let position = &block.position;
                new_visible.push(board.matrix[position.x][position.y + 1].unwrap());
                *visibility = Visibility::Hidden;
            }
        }

        for entity in new_visible {
            let (block, mut visibility) = block_query.get_mut(entity).unwrap();
            *visibility = Visibility::Visible;
        }
    }
}
