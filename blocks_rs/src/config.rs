use bevy::prelude::*;
use bevy::{prelude::*, sprite_render::MeshMaterial2d};

pub struct ConfigPlugin;

#[derive(Debug)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
}

impl From<&Window> for WindowConfig {
    fn from(w: &Window) -> Self {
        Self {
            width: w.width(),
            height: w.height(),
            title: w.title.clone(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 640.0,
            height: 960.0,
            title: "Block Game".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct BlockConfig {
    pub size: f32,
    pub center_space: f32,
    pub move_delay: f32,
}

impl Default for BlockConfig {
    fn default() -> Self {
        Self {
            size: 34.0,
            center_space: 40.0,
            move_delay: 0.5,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Configuration {
    pub window: WindowConfig,
    pub block: BlockConfig,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            block: BlockConfig::default(),
        }
    }
}

pub struct Sprite {
    pub shape: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

#[derive(Resource)]
pub struct GameSprites {
    pub board_block: Sprite,
    pub tetroid_block: Sprite,
}

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
        board_block: Sprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }))),
        },
        tetroid_block: Sprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
                alpha: 1.0,
            }))),
        },
    };

    commands.insert_resource(config);
    commands.insert_resource(game_sprites);
}
