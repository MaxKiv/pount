use bevy::prelude::*;

use crate::card::{
    bundle::{Card, CardMarker, TilePosition},
    spawn::CARD_COLORS,
};

pub fn check_wincondition(
    mut commands: Commands,
    query: Query<(&Card, &TilePosition), With<CardMarker>>,
) {
    for current_color in CARD_COLORS.iter() {
        for (card, position) in query.iter() {
            if card.color == *current_color {
                info!("{:?} at {:?}", card, position.pos());
            }
        }
    }
}
