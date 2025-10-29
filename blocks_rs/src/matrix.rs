use bevy::prelude::*;

use crate::resolution;
pub struct MatrixPlugin;

impl Plugin for MatrixPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_matrix);
    }
}

fn setup_matrix(mut commands: Commands, resolution: Res<resolution::Resolution>) {}
