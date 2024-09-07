use bevy::prelude::*;

use crate::{
    card::{
        bundle::CardMarker, sequence::CardSequence, show_next::CurrentInfoBox, spawn::CardIndex,
    },
    keys::KeyMap,
};

use super::{bundle::GameState, win_condition::PlayerWinEntity};

#[allow(clippy::too_many_arguments)]
pub fn restart_game(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<CardMarker>>,
    mut board_state: ResMut<GameState>,
    mut card_sequence: ResMut<CardSequence>,
    mut player_win_entity: ResMut<PlayerWinEntity>,
    mut card_index: ResMut<CardIndex>,
    mut current_card_entity: ResMut<CurrentInfoBox>,
    keymap: Res<KeyMap>,
) {
    if keyboard_input.just_pressed(
        keymap
            .0
            .get("restart")
            .cloned()
            .expect("Restart keymap not found"),
    ) {
        // despawn player win notification
        if let Some(entity) = player_win_entity.0 {
            info!("despawning player win notification {:?}", entity);
            commands.entity(entity).despawn_recursive();
            player_win_entity.0 = None;
        }

        // despawn all current cards, and the show_next info card
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // show_next card entity has been despawned above, set resource to None
        current_card_entity.0 = None;

        // reset the board state
        *board_state = GameState::reset();

        // generate a new cardsequence
        *card_sequence = CardSequence::generate_full_sequence();
        card_index.index = 0;
    }
}
