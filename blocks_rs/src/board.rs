use bevy::prelude::*;

use crate::block::{BaseBlock, BlockType, BoardBlock, Position, StoppedBlock, TetroidBlock};
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
    let x_max = (config.window.width / config.block.center_space).floor() as usize;
    let y_max = (config.window.height / config.block.center_space).floor() as usize;

    let mut matrix: Vec<Vec<Option<Entity>>> = vec![vec![None; y_max]; x_max];

    for x in 0..x_max {
        for y in 0..y_max {
            let base_block = BaseBlock {
                position: Position::new(x, y),
                entity: None,
            };
            let glob_cord = base_block.board_cord_to_global(&config);
            let btype = if y != y_max - 1 && x != 0 && x != x_max - 1 {
                BlockType::Tetroid
            } else {
                BlockType::Board
            };
            let bundle = BoardBlockBundle::new(&glob_cord, &sprites, &btype, base_block);
            // spawn visual bundle and attach the appropriate block component
            let id = match btype {
                BlockType::Tetroid => commands.spawn((bundle, TetroidBlock {}, StoppedBlock {})),
                BlockType::Board => commands.spawn((bundle, BoardBlock {}, StoppedBlock {})),
            }
            .id();
            matrix[x][y] = Some(id);
        }
    }

    commands.insert_resource(Board {
        height: y_max,
        width: x_max,
        matrix,
    });
}

fn draw(mut commands: Commands) {}
