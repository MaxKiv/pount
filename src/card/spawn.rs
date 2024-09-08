use bevy::{ecs::system::SystemParam, prelude::*};

pub const CARD_TEXT_DIMENSIONS: f32 = CARD_DIMENSIONS.x / 2.0;
pub const CARD_TEXT_Z_OFFSET: f32 = 0.1;
pub const CARD_DIMENSIONS: Vec2 = Vec2::new(100.0, 100.0);

const CARD_STACK_OFFSET: f32 = 5.0;

use crate::{
    asset_loader::AssetStore,
    board::{
        bundle::{GameState, BOARD_SIZE},
        win_condition::NEIGHBOURS,
    },
    card::{
        bundle::{Card, CardBundle, CardMarker},
        undo::PlacedCard,
    },
    coordinates::{ActuallyLogicalCoordinates, BoardCoordinates, LogicalCoordinates},
    keys::KeyMap,
};

use super::{sequence::CardSequence, undo::CardHistory};

/// Tracks current position in the [`CardSequence`]
#[derive(Resource, Default)]
pub struct CardIndex {
    pub index: usize,
}

/// Marker component for text
#[derive(Component, Debug)]
pub struct TextMarker;

/// Group of all system parameters used to spawn the next card
#[derive(SystemParam)]
pub struct SpawnCardContext<'w, 's> {
    pub keymap: Res<'w, KeyMap>,
    pub windows: Query<'w, 's, &'static Window>,
    pub card_index: ResMut<'w, CardIndex>,
    pub board_state: ResMut<'w, GameState>,
    pub asset_store: Res<'w, AssetStore>,
    pub card_sequence: Res<'w, CardSequence>,
    pub card_history: ResMut<'w, CardHistory>,
    pub keyboard_input: Res<'w, ButtonInput<KeyCode>>,
}

/// Spawns the next [`Card`] in the [`CardSequence`] on the tile closest to the current cursor location
pub fn spawn_card(mut commands: Commands, mut context: SpawnCardContext) {
    if should_spawn_card(&context.keyboard_input, &context.keymap) {
        let window = context.windows.single();

        // Get cursor position if cursor is in game window
        if let Some(cursor_position) = window.cursor_position() {
            let spawn_coordinates = cursor_position_to_boardcoordinates(cursor_position, window);

            // Get the next card in the sequence, if there is any left
            if let Some(next_card) = get_next_card(&context.card_index, &context.card_sequence) {
                if valid_spawn_location(&spawn_coordinates, &next_card, &context.board_state) {
                    update_board_state(
                        &spawn_coordinates,
                        &next_card,
                        &mut context.board_state,
                        &mut context.card_index,
                    );

                    render_next_card(
                        spawn_coordinates,
                        next_card,
                        &context.board_state,
                        &context.asset_store,
                        &mut commands,
                        &mut context.card_history,
                    );
                }
            }
        }
    }
}

/// Render the next [`Card`] in the [`CardSequence`] and update the [`CardHistory`]
fn render_next_card(
    spawn_coordinates: BoardCoordinates,
    next_card: Card,
    board_state: &GameState,
    asset_store: &AssetStore,
    commands: &mut Commands,
    card_history: &mut CardHistory,
) {
    // offset card placement based on number of cards currently on tile
    let actual_card_spawn = handle_cardstack_offset(&spawn_coordinates, board_state);

    // Render card
    let entity = render_card(actual_card_spawn, next_card, asset_store, commands);

    // Update card history
    update_card_history(entity, card_history, spawn_coordinates);
}

/// Update the [`CardHistory`] with the newly rendered [`Card`]s [`BoardCoordinates`] and [`Entity`]
fn update_card_history(
    entity: Entity,
    card_history: &mut CardHistory,
    spawn_coordinates: BoardCoordinates,
) {
    let board_coordinates = spawn_coordinates;
    let last_card = PlacedCard {
        entity,
        board_coordinates,
    };
    if card_history.0.is_some() {
        card_history.0.as_mut().unwrap().push(last_card);
    } else {
        card_history.0 = Some(vec![last_card]);
    }
}

/// Offset the given [`BoardCoordinates`] in the (x,y) dimension based on the number of cards already present on the [`Tile`]
fn handle_cardstack_offset(
    spawn_coordinates: &BoardCoordinates,
    board_state: &GameState,
) -> ActuallyLogicalCoordinates {
    // Get number of cards on selected tile
    let (x, y, _) = spawn_coordinates.as_xys();
    let num_cards = board_state.get_tile(x, y).cards.len();

    // Offset given spawn coordinates based on number of cards already on tile
    let mut offset_coordinates: ActuallyLogicalCoordinates = spawn_coordinates.clone().into();
    offset_coordinates.transform.translation += Transform::from_xyz(
        CARD_STACK_OFFSET * num_cards as f32,
        CARD_STACK_OFFSET * num_cards as f32,
        num_cards as f32,
    )
    .translation;

    offset_coordinates
}

/// Update the [`GameState`] with this new [`Card`] and increment [`CardIndex`]
fn update_board_state(
    spawn_coordinates: &BoardCoordinates,
    next_card: &Card,
    board_state: &mut GameState,
    card_index: &mut CardIndex,
) {
    let (x, y, _) = spawn_coordinates.as_xys();
    board_state.get_tile_mut(x, y).cards.push(*next_card);
    card_index.index += 1;
}

/// Return the tile in [`BoardCoordinates`] that the cursor is hovering over, snapping to the nearest tile
fn cursor_position_to_boardcoordinates(cursor_position: Vec2, window: &Window) -> BoardCoordinates {
    let logical_coordinates = LogicalCoordinates::from_cursor_position(cursor_position);
    let game_coordinates =
        ActuallyLogicalCoordinates::from_logical(logical_coordinates, window.height());
    let card_spawn_board_coordinates: BoardCoordinates = game_coordinates.clone().into();
    card_spawn_board_coordinates
}

/// Render a [`Card`] at [`ActuallyLogicalCoordinates`]
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
                    transform: actual_card_spawn.transform,
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
                            font_size: CARD_TEXT_DIMENSIONS,
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

/// Is the hovered tile a valid location to spawn a new card on?
fn valid_spawn_location(
    spawn_coordinates: &BoardCoordinates,
    next_card: &Card,
    board_state: &GameState,
) -> bool {
    // First card is always valid
    if board_state.empty {
        info!("valid spawn location: board empty");
        return true;
    }

    let (x, y, _) = spawn_coordinates.as_xys();
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

/// Get next [`Card`] in the [`CardSequence`]
fn get_next_card(card_index: &CardIndex, card_sequence: &Res<CardSequence>) -> Option<Card> {
    card_sequence.cards.get(card_index.index).cloned()
}

/// Should a new [`Card`] be spawned?
fn should_spawn_card(keyboard_input: &Res<ButtonInput<KeyCode>>, keymap: &Res<KeyMap>) -> bool {
    keyboard_input.just_pressed(
        keymap
            .0
            .get("spawn")
            .cloned()
            .expect("Spawn keymap not found"),
    )
}
