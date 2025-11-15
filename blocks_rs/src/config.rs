use crate::constants::{BOARD_BLOCK_COLOR, TETROID_BLOCK_COLOR};
use crate::resources::{BlockSprite, Configuration, GameSprites};
use bevy::{prelude::*, sprite_render::MeshMaterial2d};
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single().expect("Window not found");
    let config = Configuration {
        window: window.into(),
        ..Default::default()
    };

    let cube_shape = Mesh2d(meshes.add(Rectangle::new(config.block.size, config.block.size)));
    let game_sprites = GameSprites {
        board_block: BlockSprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(BOARD_BLOCK_COLOR)),
        },
        tetroid_block: BlockSprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(TETROID_BLOCK_COLOR)),
        },
    };

    commands.insert_resource(config);
    commands.insert_resource(game_sprites);
}
