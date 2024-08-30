use bevy::prelude::*;

use crate::{schedule::InGameSet, win_condition::check_wincondition};

use super::{
    sequence::generate_player_card_sequences,
    spawn::{despawn_cards, spawn_card, CardIndex},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardIndex::new(0));
        app.add_systems(Startup, generate_player_card_sequences);
        app.add_systems(
            Update,
            (spawn_card, despawn_cards)
                .chain()
                .in_set(InGameSet::MutateBoard),
        );
    }
}
