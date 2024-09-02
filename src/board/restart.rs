use bevy::prelude::*;

use crate::card::{bundle::CardMarker, sequence::CardSequence};

use super::bundle::GameBoard;

pub fn restart_game(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<CardMarker>>,
    mut board_state: ResMut<GameBoard>,
    mut card_sequence: ResMut<CardSequence>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        // despawn all current cards
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // reset the board state
        *board_state = GameBoard::reset();

        // generate a new cardsequence
        *card_sequence = CardSequence::generate();
    }
}
