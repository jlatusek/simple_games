//! Configuration setup system

use crate::constants::*;
use crate::resources::{BlockSprite, Configuration, GameSprites};
use bevy::prelude::*;

pub fn setup_config(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Get window configuration

    let window = window_query.single().expect("Window not found");
    let config = Configuration {
        window: window.into(),
        ..Default::default()
    };

    let block_mesh = Mesh2d(meshes.add(Rectangle::new(config.block.size, config.block.size)));

    let game_sprites = GameSprites {
        board_block: BlockSprite {
            shape: block_mesh.clone(),
            material: MeshMaterial2d(materials.add(BOARD_BLOCK_COLOR)),
        },
        tetroid_block: BlockSprite {
            shape: block_mesh.clone(),
            material: MeshMaterial2d(materials.add(TETROID_BLOCK_COLOR)),
        },
    };

    commands.insert_resource(config);
    commands.insert_resource(game_sprites);
}
