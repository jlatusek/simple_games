use crate::block::{BaseBlock, TetroidBlock};
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
    sprites: Res<config::GameSprites>,
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
    mut block_query: Query<(Entity, &BaseBlock, &mut Transform), With<TetroidBlock>>,
    time: Res<Time>,
    mut blocks_timer: ResMut<BlocksMoveTimer>,
    config: Res<Configuration>,
) {
    blocks_timer.timer.tick(time.delta());
    if blocks_timer.timer.is_finished() {
        for (entity, block, mut transform) in block_query.iter_mut() {
            transform.translation.y -= config.block.center_space;
        }
    }
}
