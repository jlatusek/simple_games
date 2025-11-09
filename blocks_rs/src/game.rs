use bevy::prelude::*;

use crate::{board, camera, config, sprite};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            sprite::SpritePlugin,
            // tetroid::TetroidPlugin,
            camera::CameraPlugin,
            board::BoardPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    }
}
