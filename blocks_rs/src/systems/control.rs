use crate::messages::movement::{Direction, TetroidMovementMsg};
use bevy::prelude::*;

pub fn update_control_events(
    keys: Res<ButtonInput<KeyCode>>,
    mut tetroid_message_writer: MessageWriter<TetroidMovementMsg>,
) {
    let mut tetroid_move_msg = TetroidMovementMsg::default();
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        tetroid_move_msg.move_direction = Direction::Left;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        tetroid_move_msg.move_direction = Direction::Right;
    }
    if tetroid_move_msg.move_direction == Direction::None {
        return;
    }
    tetroid_message_writer.write(tetroid_move_msg);
}
