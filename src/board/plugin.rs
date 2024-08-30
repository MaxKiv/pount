use bevy::prelude::*;

use super::bundle::setup_board;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board);
        // app.add_systems(
        //     Update,
        //     (spawn_card, despawn_cards, check_wincondition)
        //         .chain()
        //         .in_set(InGameSet::MutateBoard),
        // );
    }
}
