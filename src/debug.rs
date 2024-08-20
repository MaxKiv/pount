use bevy::prelude::*;
use std::any::type_name;

use crate::{card::Card, schedule::InGameSet};

const LOG_PERIOD: f32 = 0.5;

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
                log_entity_position::<crate::camera::CameraMarker>,
                log_entity_position::<Card>,
            )
                .in_set(InGameSet::LogState),
        );
        app.add_systems(Startup, setup_timer);
    }
}

// subsystem to log position of an entity with component T
fn log_entity_position<T: bevy::prelude::Component>(
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
