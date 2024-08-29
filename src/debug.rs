use bevy::{prelude::*, window::WindowResized};
use std::any::type_name;

use crate::{
    card::{
        bundle::{CardMarker, ColorComponent, TilePosition, Weight},
        spawn::TextMarker,
    },
    schedule::InGameSet,
};

const LOG_PERIOD: f32 = 1.0;

pub struct DebugPlugin;

#[derive(Resource)]
struct LogTimer(Timer);

fn setup_timer(mut commands: Commands) {
    commands.insert_resource(LogTimer(Timer::from_seconds(
        LOG_PERIOD,
        TimerMode::Repeating,
    )));
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // log_entity_position::<crate::camera::CameraMarker>,
                log_card,
                // log_entity_position::<CardMarker>,
                log_entity_position::<TextMarker>,
                log_window_dimensions_on_resize,
            )
                .in_set(InGameSet::LogState),
        );
        app.add_systems(Startup, setup_timer);
    }
}

fn log_card(
    query: Query<(Entity, &TilePosition, &Weight, &ColorComponent), With<CardMarker>>,
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        for (_, position, weight, color) in query.iter() {
            info!(
                "{:?} {:?} Card at position: {:?}",
                color,
                weight.weight(),
                position.pos()
            );
        }
    }
}

// subsystem to log position of an entity with component T
fn log_entity_position<T: Component>(
    query: Query<(Entity, &Transform), With<T>>,
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        for (_, transform) in query.iter() {
            info!(
                "Entity {:?} at position: {:?}",
                type_name::<T>(),
                transform.translation
            );
        }
    }
}

fn log_window_dimensions_on_resize(
    mut resize_event_reader: EventReader<WindowResized>,
    windows: Query<&Window>,
) {
    for event in resize_event_reader.read() {
        // Get the resized window by ID (typically primary window has id 0)
        if let Ok(window) = windows.get(event.window) {
            info!(
                "Window resized to: physical dimensions {:?} x {:?}, logical dimensions {:?} x {:?}",
                window.physical_width(),
                window.physical_height(),
                window.width(),
                window.height()
            );
        }
    }
}

fn log_entity_visibility<T: Component>(
    query: Query<(Entity, &Transform, &Visibility), With<T>>,
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        for (_, transform, visibility) in query.iter() {
            info!(
                "Entity {:?} at {:?} visibility: {:?}",
                type_name::<T>(),
                transform.translation,
                visibility
            );
        }
    }
}
