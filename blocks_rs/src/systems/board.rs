use crate::components::{BoardBlock, MovingBlock, Position, TetroidBlock};
use crate::messages::movement::{Direction, TetroidMovementMsg};
use crate::resources::{Board, Configuration, GameSprites};
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct BlockMoveTimer(pub Timer);

pub fn setup_board(mut commands: Commands, config: Res<Configuration>, sprites: Res<GameSprites>) {
    commands.insert_resource(BlockMoveTimer(Timer::from_seconds(
        config.block.move_delay,
        TimerMode::Repeating,
    )));

    let width = (config.window.width / config.block.center_space).floor() as usize;
    let height = (config.window.height / config.block.center_space).floor() as usize;

    let mut matrix: Vec<Vec<Option<Entity>>> = vec![vec![None; height]; width];

    for x in 0..width {
        for y in 0..height {
            let position = Position::new(x, y);
            let screen_position = position.get_global(&config);

            let is_boundary = y == height - 1 || x == 0 || x == width - 1;

            let entity = if is_boundary {
                spawn_board_block(&mut commands, position, screen_position, &sprites)
            } else {
                spawn_tetroid_block(&mut commands, position, screen_position, &sprites)
            };

            matrix[x][y] = Some(entity);
        }
    }

    commands.insert_resource(Board::new(width, height, matrix));
}

fn spawn_board_block(
    commands: &mut Commands,
    position: Position,
    screen_position: Vec2,
    sprites: &GameSprites,
) -> Entity {
    commands
        .spawn((
            BoardBlock,
            Visibility::Visible,
            position,
            sprites.board_block.shape.clone(),
            sprites.board_block.material.clone(),
            Transform::from_xyz(screen_position.x, screen_position.y, 0.0),
        ))
        .id()
}

fn spawn_tetroid_block(
    commands: &mut Commands,
    position: Position,
    screen_position: Vec2,
    sprites: &GameSprites,
) -> Entity {
    commands
        .spawn((
            TetroidBlock,
            Visibility::Hidden,
            position,
            sprites.tetroid_block.shape.clone(),
            sprites.tetroid_block.material.clone(),
            Transform::from_xyz(screen_position.x, screen_position.y, 0.0),
        ))
        .id()
}

pub fn update_board(
    mut commands: Commands,
    mut moving_block_q: Query<
        (Entity, &Position, &mut Visibility),
        (With<TetroidBlock>, With<MovingBlock>),
    >,
    mut stopped_block_q: Query<&mut Visibility, (With<TetroidBlock>, Without<MovingBlock>)>,
    board: Res<Board>,
    time: Res<Time>,
    mut move_timer: ResMut<BlockMoveTimer>,
    mut tetroid_movement_message: MessageReader<TetroidMovementMsg>,
) {
    move_timer.tick(time.delta());

    if !move_timer.is_finished() {
        return;
    }

    for (entity, position, mut visibility) in moving_block_q.iter_mut() {
        let next_y = position.y + 1;

        if !board.is_valid_position(position.x, next_y) {
            warn!(
                "Block at ({}, {}) tried to move out of bounds",
                position.x, position.y
            );
            continue;
        }

        let mut next_x = position.x;
        for event in tetroid_movement_message.read() {
            match event.move_direction {
                Direction::None => warn!("You create movement event without any direction!!!"),
                Direction::Left => next_x -= 1,
                Direction::Right => next_x += 1,
            }
        }

        let Some(next_entity) = board.get(next_x, next_y) else {
            warn!("No entity found at position ({}, {})", position.x, next_y);
            continue;
        };

        if let Ok(mut next_visibility) = stopped_block_q.get_mut(next_entity) {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<MovingBlock>();

            *next_visibility = Visibility::Visible;
            commands.entity(next_entity).insert(MovingBlock);
        }
    }
}
