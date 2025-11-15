use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Direction {
    None,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Message, Default)]
pub struct TetroidMovementMsg {
    pub move_direction: Direction,
}
