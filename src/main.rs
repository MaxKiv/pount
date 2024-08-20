mod asset_loader;
mod camera;
mod card;
mod debug;
mod movement;
mod schedule;
mod window;

use bevy::prelude::*;
use bevy::window::WindowMode::Fullscreen;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use card::CardPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use window::WindowPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.15,
        })
        .add_plugins(WindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CardPlugin)
        // .add_plugins(MovementPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}