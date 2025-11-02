use bevy::prelude::*;

use crate::{camera, config, cube, matrix, sprite};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            sprite::SpritePlugin,
            cube::CubePlugin,
            camera::CameraPlugin,
            matrix::MatrixPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    }
}
