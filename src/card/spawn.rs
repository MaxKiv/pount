use bevy::prelude::*;

pub const TEXT_DIMENSIONS: f32 = CARD_DIMENSIONS.x / 2.0;
pub const CARD_TEXT_Z_OFFSET: f32 = 0.1;

use crate::{
    asset_loader::AssetStore,
    board::{
        bundle::{GameBoard, BOARD_SIZE},
        win_condition::{StateChanged, NEIGHBOURS},
    },
    card::bundle::{Card, CardBundle, CardMarker, CARD_DIMENSIONS},
    coordinates::{ActuallyLogicalCoordinates, BoardCoordinates, LogicalCoordinates},
};

use super::sequence::CardSequence;

pub const CARD_COLORS: [Color; 4] = [
    Color::SALMON,
    Color::AQUAMARINE,
    Color::GOLD,
    Color::SEA_GREEN,
];

// Tracks current card index for the card_sequence
#[derive(Resource)]
pub struct CardIndex {
    pub index: usize,
}

impl CardIndex {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

#[derive(Component, Debug)]
pub struct TextMarker;

#[allow(clippy::too_many_arguments)]
pub fn spawn_card(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut card_index: ResMut<CardIndex>,
    mut board_state: ResMut<GameBoard>,
    asset_store: Res<AssetStore>,
    card_sequence: Res<CardSequence>,
    mut board_state_changed: ResMut<StateChanged>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            // massage coordinates a bit
            let logical_coordinates = LogicalCoordinates::from_cursor_position(cursor_position);
            let game_coordinates = ActuallyLogicalCoordinates::from_logical(
                logical_coordinates,
                windows.single().height(),
            );
            let card_spawn_board_coordinates: BoardCoordinates = game_coordinates.clone().into();

            if let Some(next_card) = get_next_card(&card_index, &card_sequence) {
                // check if player clicked a valid location to spawn a new card
                if valid_spawn_location(
                    &card_spawn_board_coordinates,
                    &next_card,
                    board_state.as_ref(),
                ) {
                    info!("spawning card at: {:?}", card_spawn_board_coordinates);

                    let mut actual_card_spawn: ActuallyLogicalCoordinates =
                        card_spawn_board_coordinates.clone().into();

                    // Update board_state
                    let (x, y, _) = card_spawn_board_coordinates.as_xys();
                    board_state.get_tile_mut(x, y).cards.push(next_card);

                    let z = board_state.get_tile(x, y).cards.len();

                    let mut spawn_transform = actual_card_spawn.transform();
                    spawn_transform.translation.z = z as f32;
                    actual_card_spawn.set_transform(spawn_transform);

                    // Notify board state has changed
                    info!("Card placed, board state changed");
                    board_state_changed.0 = true;
                    card_index.index += 1;

                    // Render card
                    let _ = render_card(actual_card_spawn, next_card, &asset_store, &mut commands);
                }
            }
        }
    }
}

pub fn render_card(
    actual_card_spawn: ActuallyLogicalCoordinates,
    card: Card,
    asset_store: &AssetStore,
    commands: &mut Commands,
) -> Entity {
    let entity = commands
        .spawn((
            CardBundle {
                card,
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: card.color,
                        custom_size: Some(CARD_DIMENSIONS),
                        ..Default::default()
                    },
                    transform: actual_card_spawn.transform(),
                    ..Default::default()
                },
            },
            CardMarker,
        ))
        // Spawn the text entity as a child of the card entity
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        card.value.to_string(),
                        TextStyle {
                            font_size: TEXT_DIMENSIONS,
                            color: Color::BLACK,
                            font: asset_store.font.clone(),
                        },
                    ),
                    // Overlay the text on the card by setting its Z value
                    transform: Transform::from_xyz(0.0, 0.0, CARD_TEXT_Z_OFFSET),
                    ..Default::default()
                },
                TextMarker,
            ));
        })
        .id();
    entity
}

// are the given new card spawning coordinates next to a
fn valid_spawn_location(
    card_spawn_board_coordinates: &BoardCoordinates,
    next_card: &Card,
    board_state: &GameBoard,
) -> bool {
    // First card is always valid
    if board_state.empty {
        info!("valid spawn location: board empty");
        return true;
    }

    let (x, y, _) = card_spawn_board_coordinates.as_xys();
    // Is there already a Card on this location, with lower value?
    if let Some(top_card) = board_state.get_tile(x, y).cards.last() {
        if next_card.value > top_card.value {
            info!("valid spawn location: {:?} > {:?}", next_card, top_card);
            return true;
        } else {
            return false;
        }
    } else {
        info!("no top card on current spawn position");
    }

    // Is there a card in on the neighbouring spots for this spawn location
    for (dx, dy) in NEIGHBOURS.iter() {
        if let Some(nx) = (x as i32).checked_add(*dx) {
            if let Some(ny) = (y as i32).checked_add(*dy) {
                let board_size: i32 = BOARD_SIZE
                    .try_into()
                    .expect("BOARD_SIZE should always be a positive integer");
                if nx >= 0 && nx < board_size && ny >= 0 && ny < board_size {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if !board_state.get_tile(nx, ny).cards.is_empty() {
                        info!(
                            "valid spawn location: {:?} at ({},{})",
                            board_state.get_tile(nx, ny).cards.last(),
                            nx,
                            ny
                        );
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn despawn_cards(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<CardMarker>>,
    mut board_state: ResMut<GameBoard>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        board_state.clear();
    }
}

fn get_next_card(card_index: &CardIndex, card_sequence: &Res<CardSequence>) -> Option<Card> {
    card_sequence.cards.get(card_index.index).cloned()
}
