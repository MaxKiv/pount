use bevy::prelude::*;

const MAX_TILE_POSITION: usize = 10;

use crate::{
    board::bundle::GameBoard,
    card::{
        bundle::{BoardPosition, CardBundle, CardMarker},
        spawn::CARD_COLORS,
    },
};

pub fn check_wincondition(mut commands: Commands, board_state: Res<GameBoard>) {
    // Loop through cards in gamestate
    // check if some(top_card) = cards.get(0)
    // check top_card neighbours, if same color
    // continue checking in that direction, if 4 consequitive WIN
}
