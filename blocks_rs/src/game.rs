use bevy::prelude::*;

use crate::{board, camera, config, tetroid};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            board::BoardPlugin,
            tetroid::TetroidPlugin,
            camera::CameraPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    }
}
