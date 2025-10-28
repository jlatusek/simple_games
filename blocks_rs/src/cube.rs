use bevy::prelude::*;

use crate::{resolution, sprite};

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cube);
    }
}

fn setup_cube(
    mut commands: Commands,
    sprites: Res<sprite::GameSprites>,
    resolution: Res<resolution::Resolution>,
) {
    commands.spawn((
        sprites.play_cube.shape.clone(),
        sprites.play_cube.material.clone(),
        Transform::from_xyz(0.0, resolution.screen_dimensions[1] / 2.0, 0.0),
    ));
}
