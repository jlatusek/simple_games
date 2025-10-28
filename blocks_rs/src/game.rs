use bevy::prelude::*;

use crate::{camera, cube, resolution, sprite};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            resolution::ResolutionPlugin,
            sprite::SpritePlugin,
            cube::CubePlugin,
            camera::CameraPlugin,
        ));
    }
}
