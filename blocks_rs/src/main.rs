use bevy::{prelude::*, window::WindowResolution};

mod board;
mod camera;
mod components;
mod config;
mod constants;
mod control;
mod game;
mod messages;
mod resources;
mod systems;
mod tetroid;

fn main() {
    println!("Blocks, blocks everywhere :D !!!");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("blocks"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: WindowResolution::new(600, 800),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            game::GamePlugin,
        ))
        .run();
}
