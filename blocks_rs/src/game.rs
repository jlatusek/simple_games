use bevy::prelude::*;

use crate::constants::BACKGROUND_COLOR;
use crate::{board, camera, config, control, tetroid};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            board::BoardPlugin,
            tetroid::TetroidPlugin,
            camera::CameraPlugin,
            control::ControlPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR));
    }
}
