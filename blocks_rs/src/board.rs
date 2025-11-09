use bevy::prelude::*;

use crate::block::{BaseBlock, BaseBlockType};
use crate::config;

pub struct BoardPlugin;

#[derive(Resource)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<BaseBlock>>,
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
}

impl BoardBlockBundle {
    fn new(position: &Vec2, sprites: &Res<config::GameSprites>, base_block: &BaseBlock) -> Self {
        let (mesh, material, visibility) = match base_block.btype {
            BaseBlockType::Tetroid => (
                sprites.play_cube.shape.clone(),
                sprites.play_cube.material.clone(),
                Visibility::Hidden,
            ),
            BaseBlockType::Board => (
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

    let mut matrix = vec![vec![BaseBlock::default(); cols]; rows];

    for (r, row) in matrix.iter_mut().enumerate() {
        for (c, obj) in row.iter_mut().enumerate() {
            obj.position.x = c as f32;
            obj.position.y = r as f32;
            if r != rows - 1 && c != 0 && c != cols - 1 {
                obj.btype = BaseBlockType::Tetroid
            }
        }
    }

    for (r, row) in matrix.iter_mut().enumerate() {
        for (c, obj) in row.iter_mut().enumerate() {
            let glob_cord = obj.board_cord_to_global(&config);
            let id = commands
                .spawn(BoardBlockBundle::new(&glob_cord, &sprites, &obj))
                .id();
            obj.entity = Some(id);
        }
    }

    commands.insert_resource(Board {
        height: rows,
        width: cols,
        matrix,
    });
}

fn draw(mut commands: Commands) {}
