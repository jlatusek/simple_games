use bevy::prelude::*;

use crate::{config, sprite};
pub struct BoardPlugin;

#[derive(Resource)]
struct Board {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<Option<Entity>>>,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_matrix)
            .add_systems(FixedUpdate, draw);
    }
}

fn setup_matrix(
    mut commands: Commands,
    config: Res<config::Configuration>,
    sprites: Res<sprite::GameSprites>,
) {
    let cols = (config.window.width / config.block.center_space).floor() as usize;
    let rows = (config.window.height / config.block.center_space).floor() as usize;

    let matrix = vec![vec![None; cols]; rows];
    commands.insert_resource(Board {
        height: rows,
        width: cols,
        matrix,
    });

    for r in (-(rows as i32) / 2)..((rows as i32) / 2) {
        commands.spawn((
            sprites.env_cube.shape.clone(),
            sprites.env_cube.material.clone(),
            Transform::from_xyz(
                -config.window.width / 2.0 + config.block.center_space / 2.0,
                r as f32 * config.block.center_space + config.block.center_space / 2.0,
                0.0,
            ),
        ));
        commands.spawn((
            sprites.env_cube.shape.clone(),
            sprites.env_cube.material.clone(),
            Transform::from_xyz(
                config.window.width / 2.0 - config.block.center_space / 2.0,
                r as f32 * config.block.center_space + config.block.center_space / 2.0,
                0.0,
            ),
        ));
    }

    for c in (-(cols as i32) / 2 + 1)..((cols as i32) / 2 - 1) {
        commands.spawn((
            sprites.env_cube.shape.clone(),
            sprites.env_cube.material.clone(),
            Transform::from_xyz(
                c as f32 * config.block.center_space + config.block.center_space / 2.0,
                -config.window.height / 2.0 + config.block.center_space / 2.0,
                0.0,
            ),
        ));
    }
}

fn draw(mut commands: Commands) {}
