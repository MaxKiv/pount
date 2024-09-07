use bevy::prelude::*;

use crate::{
    asset_loader::AssetStore,
    board::bundle::GameState,
    camera::{CAMERA_OFFSET_X, CAMERA_OFFSET_Y},
    card::spawn::render_card,
    coordinates::ActuallyLogicalCoordinates,
};

use super::{sequence::CardSequence, spawn::CardIndex};

const NEXT_CARD_X_WINDOW_WIDTH_PERCENTAGE: f32 = 0.9;
const NEXT_CARD_Y_WINDOW_WIDTH_PERCENTAGE: f32 = 0.9;
const NEXT_CARD_Z: f32 = 100.0;

#[derive(Resource)]
pub struct CurrentCardEntity(pub Option<Entity>);

pub fn show_next_card(
    card_index: Res<CardIndex>,
    card_sequence: Res<CardSequence>,
    board_state: Res<GameState>,
    asset_store: Res<AssetStore>,
    mut commands: Commands,
    windows: Query<&Window>,
    mut current_card_entity: ResMut<CurrentCardEntity>, // Inject the resource
) {
    if board_state.is_changed() {
        info!("Board state changed, showing next card");

        // Despawn the previous card entity if it exists
        if let Some(entity) = current_card_entity.0 {
            commands.entity(entity).despawn_recursive();
        }

        if let Some(next_card) = card_sequence.cards.get(card_index.index).cloned() {
            let window = windows.single();
            let x = NEXT_CARD_X_WINDOW_WIDTH_PERCENTAGE * window.width() + CAMERA_OFFSET_X;
            let y = NEXT_CARD_Y_WINDOW_WIDTH_PERCENTAGE * window.height() + CAMERA_OFFSET_Y;

            let transform = Transform::from_xyz(x, y, NEXT_CARD_Z);
            let coordinates = ActuallyLogicalCoordinates::new(transform);

            info!("Rendering next card {:?} at {:?}", next_card, coordinates);

            let next_card_entity = render_card(coordinates, next_card, &asset_store, &mut commands);
            current_card_entity.0 = Some(next_card_entity);
        }
    }
}
