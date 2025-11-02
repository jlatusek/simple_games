use bevy::prelude::*;

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

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_project_configuration);
    }
}

fn add_project_configuration(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single().expect("Window not found");
    let config = Configuration {
        window: window.into(),
        ..Default::default()
    };
    commands.insert_resource(config);
}
