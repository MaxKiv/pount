use bevy::prelude::*;

use crate::card::bundle::CardMarker;

fn check_wincondition(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<CardMarker>>,
) {
}
