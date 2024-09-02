use bevy::prelude::*;

use crate::{board::restart::restart_game, schedule::InGameSet};

use super::{
    sequence::generate_player_card_sequences,
    spawn::{spawn_card, CardIndex},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardIndex::new(0));
        app.add_systems(Startup, generate_player_card_sequences);
        app.add_systems(
            Update,
            (spawn_card, restart_game)
                .chain()
                .in_set(InGameSet::MutateBoard),
        );
    }
}
