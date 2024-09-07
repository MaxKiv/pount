use bevy::{prelude::*, window::WindowResized};

use crate::{
    asset_loader::AssetStore,
    board::bundle::GameState,
    camera::{CAMERA_OFFSET_X, CAMERA_OFFSET_Y},
    card::spawn::{render_card, TextMarker, CARD_TEXT_DIMENSIONS},
    coordinates::ActuallyLogicalCoordinates,
};

use super::{sequence::CardSequence, spawn::CardIndex};

const NEXT_CARD_X_WINDOW_WIDTH_PERCENTAGE: f32 = 0.9;
const NEXT_CARD_Y_WINDOW_WIDTH_PERCENTAGE: f32 = 0.8;
const NEXT_CARD_Z: f32 = 100.0;
const NEXT_CARD_TEXT_Y_OFFSET: f32 = 80.0;

// You shall be known as the infobox, purveyor of informations, clairvoyeur of the card_sequence
#[derive(Resource)]
pub struct CurrentInfoBox(pub Option<Entity>);

#[allow(clippy::too_many_arguments)]
pub fn show_next_card(
    card_index: Res<CardIndex>,
    card_sequence: Res<CardSequence>,
    board_state: Res<GameState>,
    asset_store: Res<AssetStore>,
    mut commands: Commands,
    mut current_infobox: ResMut<CurrentInfoBox>,
    resize_event_reader: EventReader<WindowResized>,
    windows: Query<&Window>,
) {
    if board_state.is_changed() || !resize_event_reader.is_empty() {
        info!("Board state changed, showing next card");

        let window = windows.single();

        render_next_card_infobox(
            card_index,
            card_sequence,
            window,
            &asset_store,
            &mut commands,
            &mut current_infobox,
        );
    }
}

fn render_next_card_infobox(
    card_index: Res<CardIndex>,
    card_sequence: Res<CardSequence>,
    window: &Window,
    asset_store: &AssetStore,
    commands: &mut Commands,
    current_infobox: &mut CurrentInfoBox,
) {
    // Despawn the previous infobox if it exists
    if let Some(entity) = current_infobox.0 {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(next_card) = card_sequence.cards.get(card_index.index).cloned() {
        let x = NEXT_CARD_X_WINDOW_WIDTH_PERCENTAGE * window.width() + CAMERA_OFFSET_X;
        let y = NEXT_CARD_Y_WINDOW_WIDTH_PERCENTAGE * window.height() + CAMERA_OFFSET_Y;

        let transform = Transform::from_xyz(x, y, NEXT_CARD_Z);
        let coordinates = ActuallyLogicalCoordinates::new(transform);

        info!("Rendering next card {:?} at {:?}", next_card, coordinates);

        let next_card_entity = render_card(coordinates, next_card, asset_store, commands);
        commands.entity(next_card_entity).with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "Next card",
                        TextStyle {
                            font_size: CARD_TEXT_DIMENSIONS,
                            color: Color::BLACK,
                            font: asset_store.font.clone(),
                        },
                    ),
                    // Overlay the text on the card by setting its Z value
                    transform: Transform::from_xyz(0.0, NEXT_CARD_TEXT_Y_OFFSET, NEXT_CARD_Z),
                    ..Default::default()
                },
                TextMarker,
            ));
        });

        //
        current_infobox.0 = Some(next_card_entity);
    }
}
