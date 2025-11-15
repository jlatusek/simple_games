use bevy::prelude::*;

use crate::block::{BoardBlock, MovingBlock, Position, TetroidBlock};
use crate::config;

pub struct BoardPlugin;

#[derive(Resource)]
struct BlocksMoveTimer {
    timer: Timer,
}

#[derive(Resource)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<Option<Entity>>>,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board)
            .add_systems(FixedUpdate, update_board);
    }
}

fn setup_board(
    mut commands: Commands,
    config: Res<config::Configuration>,
    sprites: Res<config::GameSprites>,
) {
    commands.insert_resource(BlocksMoveTimer {
        timer: Timer::from_seconds(config.block.move_delay, TimerMode::Repeating),
    });
    let x_max = (config.window.width / config.block.center_space).floor() as usize;
    let y_max = (config.window.height / config.block.center_space).floor() as usize;

    let mut matrix: Vec<Vec<Option<Entity>>> = vec![vec![None; y_max]; x_max];

    for x in 0..x_max {
        for y in 0..y_max {
            let position = Position::new(x, y);
            let glob_cord = position.get_global(&config);
            let id: Entity;
            if y != y_max - 1 && x != 0 && x != x_max - 1 {
                id = commands
                    .spawn((
                        TetroidBlock {},
                        Visibility::Hidden,
                        position,
                        sprites.tetroid_block.shape.clone(),
                        sprites.tetroid_block.material.clone(),
                        Transform::from_xyz(glob_cord.x, glob_cord.y, 0.0),
                    ))
                    .id();
            } else {
                id = commands
                    .spawn((
                        BoardBlock {},
                        Visibility::Visible,
                        position,
                        sprites.board_block.shape.clone(),
                        sprites.board_block.material.clone(),
                        Transform::from_xyz(glob_cord.x, glob_cord.y, 0.0),
                    ))
                    .id();
            };
            matrix[x][y] = Some(id);
        }
    }

    commands.insert_resource(Board {
        height: y_max,
        width: x_max,
        matrix,
    });
}

fn update_board(
    mut commands: Commands,
    mut moving_block_q: Query<
        (Entity, &Position, &mut Visibility),
        (With<TetroidBlock>, With<MovingBlock>),
    >,
    mut stopped_block_q: Query<&mut Visibility, (With<TetroidBlock>, Without<MovingBlock>)>,
    board: Res<Board>,
    time: Res<Time>,
    mut blocks_timer: ResMut<BlocksMoveTimer>,
) {
    blocks_timer.timer.tick(time.delta());

    if !blocks_timer.timer.is_finished() {
        return;
    }
    for (entity, position, mut visibility) in moving_block_q.iter_mut() {
        let new = board.matrix[position.x][position.y + 1].unwrap();
        if let Ok(mut next_visibility) = stopped_block_q.get_mut(new) {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<MovingBlock>();

            *next_visibility = Visibility::Visible;
            commands.entity(new).insert(MovingBlock {});
        }
    }
}
