use bevy::prelude::*;

use crate::config::Configuration;
use crate::{config, sprite};

pub struct CubePlugin;

#[derive(Resource)]
struct BlocksMoveTimer {
    timer: Timer,
}

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_cube)
            .add_systems(FixedUpdate, update_cube);
    }
}

#[derive(Component)]
struct BaseBlock;

fn setup_cube(
    mut commands: Commands,
    sprites: Res<sprite::GameSprites>,
    config: Res<config::Configuration>,
) {
    commands.spawn((
        BaseBlock {},
        sprites.play_cube.shape.clone(),
        sprites.play_cube.material.clone(),
        Transform::from_xyz(
            0.0,
            config.window.height / 2.0 - config.block.center_space / 2.0,
            0.0,
        ),
    ));
    commands.insert_resource(BlocksMoveTimer {
        timer: Timer::from_seconds(config.block.move_delay, TimerMode::Repeating),
    });
}

fn update_cube(
    mut block_query: Query<(Entity, &BaseBlock, &mut Transform)>,
    time: Res<Time>,
    mut blocks_timer: ResMut<BlocksMoveTimer>,
    config: Res<Configuration>,
) {
    blocks_timer.timer.tick(time.delta());
    if blocks_timer.timer.is_finished() {
        for (entity, alien, mut transform) in block_query.iter_mut() {
            transform.translation.y -= config.block.center_space;
        }
    }
}
