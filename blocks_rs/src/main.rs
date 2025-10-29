use bevy::{prelude::*, window::WindowResolution};

mod camera;
mod cube;
mod game;
mod matrix;
mod resolution;
mod sprite;

fn main() {
    println!("Blocks, blocks everywhere :D !!!");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("blocks"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: WindowResolution::new(640, 800),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            game::GamePlugin,
        ))
        .run();
}
