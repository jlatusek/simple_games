use bevy::prelude::*;

pub struct ConfigPlugin;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
}
#[derive(Resource, Debug)]
pub struct Configuration {
    pub window: WindowConfig,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_project_configuration);
    }
}

fn add_project_configuration(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single().expect("Window not found");
    let config = Configuration {
        window: WindowConfig {
            height: window.height(),
            width: window.width(),
            title: window.title.clone(),
        },
    };
    commands.insert_resource(config);
}
