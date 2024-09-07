use bevy::prelude::*;

use crate::{board::restart::restart_game, schedule::InGameSet};

use super::{
    sequence::generate_card_sequences,
    show_next::{show_next_card, CurrentInfoBox},
    spawn::{spawn_card, CardIndex},
    undo::{undo_last_move, CardHistory},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardIndex::new(0));
        app.insert_resource(CurrentInfoBox(None));
        app.insert_resource(CardHistory(None));
        app.add_systems(Startup, generate_card_sequences);
        app.add_systems(
            Update,
            (spawn_card, undo_last_move, restart_game, show_next_card)
                .chain()
                .in_set(InGameSet::MutateBoard),
        );
    }
}
