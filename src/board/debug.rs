use bevy::prelude::*;

use crate::debug::LogTimer;

use super::bundle::GameState;

pub fn log_gamestate(time: Res<Time>, mut timer: ResMut<LogTimer>, board_state: Res<GameState>) {
    if timer.0.tick(time.delta()).finished() {
        // for (y, tiles) in board_state.board().iter().enumerate() {
        //     for (x, tile) in tiles.iter().enumerate() {
        //         if let Some(top_card) = tile.cards.last() {
        //             let board_coordinates = BoardCoordinates::from_xyz(x, y, tile.cards.len());
        //             info!("{:?} at position: {:?}", top_card, board_coordinates);
        //         }
        //     }
        // }
        info!("{:?}", *board_state);
    }
}
