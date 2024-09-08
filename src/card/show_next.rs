use bevy::window::WindowResized;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    asset_loader::AssetStore,
    board::bundle::GameState,
    camera::{CAMERA_OFFSET_X, CAMERA_OFFSET_Y},
    card::spawn::{render_card, TextMarker, CARD_TEXT_DIMENSIONS},
    coordinates::ActuallyLogicalCoordinates,
};

use super::bundle::Card;
use super::{sequence::CardSequence, spawn::CardIndex};

/// Percentage of the window width at width the next card infobox is rendered
const NEXT_CARD_WINDOW_WIDTH_PERCENTAGE: f32 = 0.9;
/// Percentage of the window height at width the next card infobox is rendered
const NEXT_CARD_WINDOW_HEIGHT_PERCENTAGE: f32 = 0.8;
/// Z offset at which the next card infobox is rendered
const NEXT_CARD_Z: f32 = 100.0;
/// Y offset at which the next card infobox text is rendered, relative to the infobox transform
const NEXT_CARD_TEXT_Y_OFFSET: f32 = 80.0;

/// You shall be known as the infobox, purveyor of informations, clairvoyeur of the card_sequence
#[derive(Resource)]
pub struct CurrentInfoBox(pub Option<Entity>);

/// Group of all system parameters used to show the next card infobox
#[derive(SystemParam)]
pub struct NextCardInfoContext<'w, 's> {
    pub card_index: Res<'w, CardIndex>,
    pub card_sequence: Res<'w, CardSequence>,
    pub board_state: Res<'w, GameState>,
    pub asset_store: Res<'w, AssetStore>,
    pub current_infobox: ResMut<'w, CurrentInfoBox>,
    pub windows: Query<'w, 's, &'static Window>,
    pub resize_event_reader: EventReader<'w, 's, WindowResized>,
}

/// Sytem to show players the next card in the sequence
/// Without this they will not know what card they are about to place
pub fn show_infobox(mut context: NextCardInfoContext, mut commands: Commands) {
    if should_refresh_infobox(context.board_state, &context.resize_event_reader) {
        let window = context.windows.single();

        update_infobox(
            context.card_index,
            context.card_sequence,
            window,
            &context.asset_store,
            &mut commands,
            &mut context.current_infobox,
        );
    }
}

/// helper function to update the infobox: despawn the old and render the new
fn update_infobox(
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

    // Spawn the new infobox, but only if the card sequence is not empty
    if let Some(next_card) = card_sequence.cards.get(card_index.index).cloned() {
        let x = NEXT_CARD_WINDOW_WIDTH_PERCENTAGE * window.width() + CAMERA_OFFSET_X;
        let y = NEXT_CARD_WINDOW_HEIGHT_PERCENTAGE * window.height() + CAMERA_OFFSET_Y;
        let transform = Transform::from_xyz(x, y, NEXT_CARD_Z);
        let coordinates = ActuallyLogicalCoordinates::new(transform);

        info!("Rendering next card {:?} at {:?}", next_card, coordinates);

        render_infobox(
            coordinates,
            next_card,
            asset_store,
            commands,
            current_infobox,
        );
    }
}

/// Renders a new infobox showing the [next_card] at [coordinates]
fn render_infobox(
    coordinates: ActuallyLogicalCoordinates,
    next_card: Card,
    asset_store: &AssetStore,
    commands: &mut Commands,
    current_infobox: &mut CurrentInfoBox,
) {
    let mut new_infobox = render_card(coordinates, next_card, asset_store, commands);
    new_infobox = commands
        .entity(new_infobox)
        .with_children(|parent| {
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
                    // Overlay the text on the card
                    transform: Transform::from_xyz(0.0, NEXT_CARD_TEXT_Y_OFFSET, NEXT_CARD_Z),
                    ..Default::default()
                },
                TextMarker,
            ));
        })
        .id();

    current_infobox.0 = Some(new_infobox);
}

/// Update infobox when board state changes or window is resized
fn should_refresh_infobox(
    board_state: Res<GameState>,
    resize_event_reader: &EventReader<WindowResized>,
) -> bool {
    board_state.is_changed() || !resize_event_reader.is_empty()
}
