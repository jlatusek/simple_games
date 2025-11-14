use bevy::prelude::*;

use crate::block::{BaseBlock, BlockType, BoardBlock, TetroidBlock};
use crate::config;

pub struct BoardPlugin;

#[derive(Resource)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<Option<Entity>>>,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board)
            .add_systems(FixedUpdate, draw);
    }
}

#[derive(Bundle)]
pub struct BoardBlockBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    visibility: Visibility,
    base_block: BaseBlock,
}

impl BoardBlockBundle {
    fn new(
        position: &Vec2,
        sprites: &Res<config::GameSprites>,
        btype: &BlockType,
        base_block: BaseBlock,
    ) -> Self {
        let (mesh, material, visibility) = match btype {
            BlockType::Tetroid => (
                sprites.play_cube.shape.clone(),
                sprites.play_cube.material.clone(),
                Visibility::Hidden,
            ),
            BlockType::Board => (
                sprites.env_cube.shape.clone(),
                sprites.env_cube.material.clone(),
                Visibility::Visible,
            ),
        };
        Self {
            mesh,
            material,
            visibility,
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            base_block,
        }
    }

    fn add_entry(self: &Self) {}
}

fn setup_board(
    mut commands: Commands,
    config: Res<config::Configuration>,
    sprites: Res<config::GameSprites>,
) {
    let cols = (config.window.width / config.block.center_space).floor() as usize;
    let rows = (config.window.height / config.block.center_space).floor() as usize;

    let mut matrix: Vec<Vec<Option<Entity>>> = vec![vec![None; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let base_block = BaseBlock {
                position: Vec2::new(c as f32, r as f32),
                entity: None,
            };
            let glob_cord = base_block.board_cord_to_global(&config);
            let btype = if r != rows - 1 && c != 0 && c != cols - 1 {
                BlockType::Tetroid
            } else {
                BlockType::Board
            };
            let bundle = BoardBlockBundle::new(&glob_cord, &sprites, &btype, base_block);
            // spawn visual bundle and attach the appropriate block component
            let id = match btype {
                BlockType::Tetroid => commands.spawn((bundle, TetroidBlock {})),
                BlockType::Board => commands.spawn((bundle, BoardBlock {})),
            }
            .id();
            matrix[r][c] = Some(id);
        }
    }

    commands.insert_resource(Board {
        height: rows,
        width: cols,
        matrix,
    });
}

fn draw(mut commands: Commands) {}
