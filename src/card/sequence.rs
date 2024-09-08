use bevy::prelude::*;
use rand::prelude::SliceRandom;

use super::bundle::{Card, CARD_COLORS};

const CARD_VALUES: std::ops::RangeInclusive<i32> = 1..=9;
const VALUES_PER_COLOR: usize = 2;

/// A Sequence of [`Card`] used to draw from each turn
#[derive(Debug, Resource, Default)]
pub struct CardSequence {
    pub cards: Vec<Card>,
}

impl CardSequence {
    /// Generate a sequence of cards of given color with randomized values
    fn generate_color(color: Color) -> Vec<Card> {
        let mut rng = rand::thread_rng();

        // Duplicate the range so we get the right number of values per color
        let mut numbers: Vec<i32> = Vec::new();
        for _ in 0..VALUES_PER_COLOR {
            numbers.extend(CARD_VALUES.clone());
        }

        // Map the range into a sequence of [`Card`]
        let mut cards: Vec<Card> = numbers
            .into_iter()
            .map(|value| Card { value, color })
            .collect();

        // Randomize the [`CardSequence`]
        cards.shuffle(&mut rng);

        cards
    }

    /// Generate a CardSequence for a single player
    pub fn generate_player_sequence(color_1: Color, color_2: Color) -> Self {
        let mut rng = rand::thread_rng();
        let mut sequence = CardSequence::default();

        for card in Self::generate_color(color_1) {
            sequence.cards.push(card);
        }
        for card in Self::generate_color(color_2) {
            sequence.cards.push(card);
        }

        sequence.cards.shuffle(&mut rng);
        sequence
    }

    /// Generate a CardSequence
    pub fn generate_full_sequence() -> Self {
        let mut sequences: Vec<CardSequence> = Vec::new();

        for chunk in CARD_COLORS.chunks(2) {
            sequences.push(CardSequence::generate_player_sequence(chunk[0], chunk[1]));
        }

        CardSequence::flatten_interleaved(sequences)
    }

    /// Turn a [`Vec<CardSequence>`] into a [`CardSequence`] by interleaving each [`CardSequence`] in the [`Vec`]
    fn flatten_interleaved(vec: Vec<CardSequence>) -> CardSequence {
        let mut out = CardSequence::default();

        let max_len = vec.iter().map(|seq| seq.cards.len()).max().unwrap_or(0);

        for i in 0..max_len {
            for seq in vec.iter() {
                if let Some(card) = seq.cards.get(i) {
                    out.cards.push(*card);
                }
            }
        }

        out
    }
}

/// System to generate a new [`CardSequence`]
pub fn generate_card_sequences(mut commands: Commands) {
    let sequence = CardSequence::generate_full_sequence();
    // TODO extend this for different number of players
    info!("generated card sequence {:?}", sequence);
    commands.insert_resource(sequence);
}
