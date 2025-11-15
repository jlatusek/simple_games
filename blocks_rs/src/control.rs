use crate::messages::movement::TetroidMovementMsg;
use crate::systems::control::update_control_events;
use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_control_events)
            .add_message::<TetroidMovementMsg>();
    }
}
