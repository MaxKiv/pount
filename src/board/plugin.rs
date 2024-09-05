use bevy::prelude::*;

use crate::schedule::InGameSet;

use super::{
    bundle::setup_board,
    win_condition::{check_wincondition, PlayerWinEntity, StateChanged},
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StateChanged(true)); // Starting with true to show next card
        app.insert_resource(PlayerWinEntity(None));
        app.add_systems(Startup, setup_board);
        app.add_systems(
            Update,
            check_wincondition.in_set(InGameSet::CheckWincondition),
        );
    }
}
