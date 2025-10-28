use bevy::prelude::*;

pub struct SpritePlugin;

pub struct Atlas {
    pub image: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct Sprites {
    pub cube: Atlas,
}

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
}
