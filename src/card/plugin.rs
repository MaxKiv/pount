use bevy::prelude::*;

use crate::schedule::InGameSet;

use super::{
    sequence::generate_player_card_sequences,
    spawn::{despawn_cards, spawn_card, ColorIndex},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColorIndex::new(0));
        app.add_systems(Startup, generate_player_card_sequences);
        app.add_systems(
            Update,
            (spawn_card, despawn_cards)
                .chain()
                .in_set(InGameSet::MutateBoard),
        );
    }
}
