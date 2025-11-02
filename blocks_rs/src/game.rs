use bevy::prelude::*;

use crate::{camera, config, cube, sprite};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            sprite::SpritePlugin,
            cube::CubePlugin,
            camera::CameraPlugin,
        ));
    }
}
