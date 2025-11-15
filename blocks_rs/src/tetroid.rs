use crate::block::{BaseBlock, MovingBlock, StoppedBlock, TetroidBlock};
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
    commands
        .entity(entity)
        .remove::<StoppedBlock>()
        .insert(MovingBlock {});
}

fn update_cube(
    mut commands: Commands,
    mut moving_block_q: Query<
        (Entity, &mut BaseBlock, &mut Visibility),
        (With<TetroidBlock>, With<MovingBlock>, Without<StoppedBlock>),
    >,
    mut stopped_block_q: Query<
        (&mut BaseBlock, &mut Visibility),
        (With<TetroidBlock>, With<StoppedBlock>, Without<MovingBlock>),
    >,
    board: Res<Board>,
    time: Res<Time>,
    mut blocks_timer: ResMut<BlocksMoveTimer>,
    config: Res<Configuration>,
) {
    blocks_timer.timer.tick(time.delta());

    if blocks_timer.timer.is_finished() {
        let fallen: i32 = 0;
        for (entity, block, mut visibility) in moving_block_q.iter_mut() {
            if visibility.eq(&Visibility::Visible) {
                let position = &block.position;
                let new = board.matrix[position.x][position.y + 1].unwrap();
                if let Ok((neb_block, mut new_visibility)) = stopped_block_q.get_mut(new) {
                    *visibility = Visibility::Hidden;
                    *new_visibility = Visibility::Visible;
                    commands
                        .entity(new)
                        .remove::<StoppedBlock>()
                        .insert(MovingBlock {});
                    commands
                        .entity(entity)
                        .remove::<MovingBlock>()
                        .insert(StoppedBlock {});
                }
            }
        }
    }
}
