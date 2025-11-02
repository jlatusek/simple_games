use bevy::{prelude::*, window::WindowResolution};

mod board;
mod camera;
mod config;
mod game;
mod sprite;
mod tetroid;

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
