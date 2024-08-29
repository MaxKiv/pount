use bevy::prelude::*;

pub const TEXT_DIMENSIONS: f32 = CARD_DIMENSIONS.x / 2.0;
pub const TEXT_Z_OFFSET: f32 = 0.1;

use crate::{
    asset_loader::AssetStore,
    card::bundle::{Card, CardBundle, CardMarker, TilePosition, CARD_DIMENSIONS},
    coordinates::{ActuallyLogicalCoordinates, LogicalCoordinates, TileCoordinates},
};

use super::sequence::CardSequence;

pub const CARD_COLORS: [Color; 4] = [
    Color::SALMON,
    Color::AQUAMARINE,
    Color::GOLD,
    Color::SEA_GREEN,
];

#[derive(Resource)]
pub struct CardIndex {
    index: usize,
}

impl CardIndex {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

#[derive(Component, Debug)]
pub struct TextMarker;

pub fn spawn_card(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut card_index: ResMut<CardIndex>,
    asset_store: Res<AssetStore>,
    card_sequence: Res<CardSequence>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            let logical_coordinates = LogicalCoordinates::from_cursor_position(cursor_position);
            let game_coordinates = ActuallyLogicalCoordinates::from_logical(
                logical_coordinates,
                windows.single().height(),
            );
            let card_spawn_tile_coordinates: TileCoordinates = game_coordinates.clone().into();
            info!("spawning card at: {:?}", card_spawn_tile_coordinates);

            let actual_card_spawn: ActuallyLogicalCoordinates =
                card_spawn_tile_coordinates.clone().into();

            if let Some(card) = get_next_card(&mut card_index, &card_sequence) {
                let color = card.color;
                let value = card.value;
                commands
                    .spawn((
                        CardBundle {
                            card,
                            position: TilePosition::new(card_spawn_tile_coordinates.clone()),

                            sprite: SpriteBundle {
                                sprite: Sprite {
                                    color,
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
                            // commands.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    value.to_string(),
                                    TextStyle {
                                        font_size: TEXT_DIMENSIONS,
                                        color: Color::BLACK,
                                        font: asset_store.font.clone(),
                                    },
                                ),
                                // Overlay the text on the card by setting its Z value 0.1 higher
                                transform: Transform::from_xyz(0.0, 0.0, TEXT_Z_OFFSET),
                                ..Default::default()
                            },
                            TextMarker,
                        ));
                    });
            }
        }
    }
}

pub fn despawn_cards(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<CardMarker>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn get_next_card(card_index: &mut CardIndex, card_sequence: &Res<CardSequence>) -> Option<Card> {
    let card = card_sequence.cards.get(card_index.index).cloned();
    card_index.index += 1;
    card
}
