use bevy::prelude::*;

use crate::{coordinates::BoardCoordinates, debug::LogTimer};

use super::bundle::GameBoard;

// subsystem to log position of an entity with component T
pub fn log_gamestate(time: Res<Time>, mut timer: ResMut<LogTimer>, board_state: Res<GameBoard>) {
    if timer.0.tick(time.delta()).finished() {
        for (y, tiles) in board_state.board().iter().enumerate() {
            for (x, tile) in tiles.iter().enumerate() {
                if let Some(top_card) = tile.cards.last() {
                    let board_coordinates = BoardCoordinates::from_xyz(x, y, tile.cards.len());
                    info!("{:?} at position: {:?}", top_card, board_coordinates);
                }
            }
        }
    }
}
