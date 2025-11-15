use crate::systems::setup_tetroid;
use bevy::prelude::*;

pub struct TetroidPlugin;

impl Plugin for TetroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_tetroid);
    }
}
