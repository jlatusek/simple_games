use bevy::{post_process::bloom::Bloom, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera::default(), Bloom::NATURAL));
}
