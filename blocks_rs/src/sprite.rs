use bevy::{prelude::*, sprite_render::MeshMaterial2d};

const CUBE_SIZE: f32 = 40.0;

pub struct SpritePlugin;

pub struct Sprite {
    pub shape: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

#[derive(Resource)]
pub struct GameSprites {
    pub env_cube: Sprite,
    pub play_cube: Sprite,
}

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cube_shape = Mesh2d(meshes.add(Rectangle::new(CUBE_SIZE, CUBE_SIZE)));
    let game_sprites = GameSprites {
        env_cube: Sprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }))),
        },
        play_cube: Sprite {
            shape: cube_shape.clone(),
            material: MeshMaterial2d(materials.add(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
                alpha: 1.0,
            }))),
        },
    };
    commands.insert_resource(game_sprites);
}
