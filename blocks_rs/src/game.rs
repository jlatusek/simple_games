use bevy::prelude::*;

use crate::resolution;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((resolution::ResolutionPlugin,));
    }
}
