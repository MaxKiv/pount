use bevy::prelude::*;

use crate::{card::Card, schedule::InGameSet};

// const VELOCITY: Vec3 = Vec3::new(0.01, 0.01, 0.01);
const POSITION: Vec3 = Vec3::new(1.0, 1.0, 1.0);

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Resource)]
struct MoveTimer(Timer);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_entities.in_set(InGameSet::MutateBoard));
        app.add_systems(Startup, setup_timer);
    }
}

fn setup_timer(mut commands: Commands) {
    // Timer set to 1 second
    commands.insert_resource(MoveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
}

fn move_entities(
    mut query: Query<&mut Transform, With<Card>>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
) {
    // Update the timer with the delta time
    if timer.0.tick(time.delta()).finished() {
        for mut transform in query.iter_mut() {
            transform.translation *= Vec3::new(-1.0, 1.0, 1.0);
        }
    }
}
