use bevy::prelude::*;

use crate::{config, sprite};

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_cube);
    }
}

fn setup_cube(
    mut commands: Commands,
    sprites: Res<sprite::GameSprites>,
    config: Res<config::Configuration>,
) {
    commands.spawn((
        sprites.play_cube.shape.clone(),
        sprites.play_cube.material.clone(),
        Transform::from_xyz(0.0, config.window.height / 2.0, 0.0),
    ));
}
