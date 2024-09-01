mod asset_loader;
pub mod board;
mod camera;
pub mod card;
mod coordinates;
mod debug;
mod movement;
mod schedule;
mod window;

use bevy::prelude::*;
use board::plugin::BoardPlugin;

use crate::card::plugin::CardPlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
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
        .add_plugins(BoardPlugin)
        .add_plugins(CardPlugin)
        // .add_plugins(MovementPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
