use bevy::prelude::*;

use crate::{
    board::{bundle::GameState, win_condition::PlayerWinEntity},
    coordinates::BoardCoordinates,
};

use super::spawn::CardIndex;

pub struct LastPlacedCard {
    pub board_coordinates: BoardCoordinates,
    pub entity: Entity,
}

// TODO Bevy exposes thing that are way more useful for this
#[derive(Resource)]
pub struct LastPlacedCardRes(pub Option<LastPlacedCard>);

#[allow(clippy::too_many_arguments)]
pub fn undo_last_move(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut card_index: ResMut<CardIndex>,
    mut board_state: ResMut<GameState>,
    mut last_placed_card: ResMut<LastPlacedCardRes>,
    mut player_win_entity: ResMut<PlayerWinEntity>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        if let Some(LastPlacedCard {
            board_coordinates,
            entity,
        }) = &mut last_placed_card.0
        {
            let (x, y, _) = board_coordinates.as_xys();
            // despawn player win notification if previous move won the game
            if let Some(entity) = player_win_entity.0 {
                info!("despawning player win notification {:?}", entity);
                commands.entity(entity).despawn_recursive();
                player_win_entity.0 = None;
            }

            // Remove last placed card from gamestate
            board_state
                .get_tile_mut(x, y)
                .cards
                .pop()
                .expect("Attempting to remove the previously placed card, but no card found");

            // Despawn last entity
            commands.entity(*entity).despawn_recursive();

            // Decrement card index
            card_index.index -= 1;
        }
    }
}
