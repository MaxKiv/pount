use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowMode::Fullscreen;

use crate::keys::KeyMap;
use crate::schedule::InGameSet;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                title: "pount".into(),
                mode: Fullscreen,
                ..default()
            }),
            ..default()
        }));

        app.add_systems(Update, handle_menu_keys.in_set(InGameSet::HandleMenu));
    }
}

fn handle_menu_keys(
    mut exit: EventWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>,
    keymap: Res<KeyMap>,
) {
    if input.just_pressed(
        keymap
            .0
            .get("quit")
            .cloned()
            .expect("Quit keymap not found"),
    ) {
        exit.send(AppExit);
    }
}
