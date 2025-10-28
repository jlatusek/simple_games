use bevy::prelude::*;

use crate::resolution;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cube);
    }
}

fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<resolution::Resolution>,
) {
    let cube_shape = meshes.add(Rectangle::new(40.0, 40.0));
    println!(
        "{} x {}",
        resolution.screen_dimensions[0], resolution.screen_dimensions[1]
    );
    commands.spawn((
        Mesh2d(cube_shape),
        MeshMaterial2d(materials.add(Color::Srgba(Srgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }))),
        Transform::from_xyz(0.0, resolution.screen_dimensions[1] / 2.0, 0.0),
    ));
}
