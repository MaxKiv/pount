use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    board::{bundle::GameState, win_condition::PlayerWinEntity},
    coordinates::BoardCoordinates,
    keys::KeyMap,
};

use super::spawn::CardIndex;

pub struct PlacedCard {
    pub board_coordinates: BoardCoordinates,
    pub entity: Entity,
}

#[derive(Resource)]
pub struct CardHistory(pub Option<Vec<PlacedCard>>);

#[derive(SystemParam)]
pub struct UndoContext<'w> {
    pub card_index: ResMut<'w, CardIndex>,
    pub board_state: ResMut<'w, GameState>,
    pub card_history: ResMut<'w, CardHistory>,
    pub player_win_entity: ResMut<'w, PlayerWinEntity>,
    pub keymap: Res<'w, KeyMap>,
}

pub fn undo_last_move(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut context: UndoContext,
) {
    if should_undo(&keyboard_input, &context.keymap) {
        if let Some(placed_cards) = context.card_history.0.as_mut() {
            handle_undo(
                &mut commands,
                placed_cards,
                &mut context.board_state,
                &mut context.player_win_entity,
                &mut context.card_index,
            );
        }
    }
}

fn handle_undo(
    commands: &mut Commands,
    placed_cards: &mut Vec<PlacedCard>,
    board_state: &mut GameState,
    player_win_entity: &mut PlayerWinEntity,
    card_index: &mut CardIndex,
) {
    if let Some(PlacedCard {
        board_coordinates,
        entity,
    }) = placed_cards.pop()
    {
        // despawn player win notification if previous move won the game
        despawn_win_notification(commands, player_win_entity);

        // Remove last placed card from gamestate
        undo_card_placement(board_state, board_coordinates, placed_cards);

        // Despawn last entity
        commands.entity(entity).despawn_recursive();

        // Decrement card index
        card_index.index -= 1;
    }
}

fn undo_card_placement(
    board_state: &mut GameState,
    board_coordinates: BoardCoordinates,
    placed_cards: &[PlacedCard],
) {
    let (x, y, _) = board_coordinates.as_xys();

    board_state
        .get_tile_mut(x, y)
        .cards
        .pop()
        .expect("Attempting to remove the previously placed card, but no card found");

    // Undo of the first move: update board state empty flag
    if placed_cards.is_empty() {
        info!("setting board state empty");
        board_state.empty = true;
    }
}

fn despawn_win_notification(commands: &mut Commands, player_win_entity: &mut PlayerWinEntity) {
    if let Some(entity) = player_win_entity.0 {
        info!("despawning player win notification {:?}", entity);
        commands.entity(entity).despawn_recursive();
        player_win_entity.0 = None;
    }
}

fn should_undo(keyboard_input: &ButtonInput<KeyCode>, keymap: &KeyMap) -> bool {
    keyboard_input.just_pressed(
        keymap
            .0
            .get("undo")
            .cloned()
            .expect("Undo keymap not found"),
    )
}
