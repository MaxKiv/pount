use crate::card::spawn::CARD_COLORS;

use bevy::prelude::*;
use rand::prelude::SliceRandom;

use super::bundle::Card;

const CARD_VALUES: std::ops::RangeInclusive<i32> = 1..=9;
const NUM_CARDS_PER_VALUE: usize = 2;

#[derive(Debug, Resource)]
pub struct CardSequence {
    pub cards: Vec<Card>,
}

impl CardSequence {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    // Generate a sequence of cards of given color with randomized values
    fn generate_color(color: Color) -> Vec<Card> {
        let mut rng = rand::thread_rng();

        // Collect the numbers into a vector by repeating CARD_VALUES NUM_CARDS_PER_VALUE times
        let mut numbers: Vec<i32> = Vec::new();
        for _ in 0..NUM_CARDS_PER_VALUE {
            numbers.extend(CARD_VALUES.clone());
        }

        let mut cards: Vec<Card> = numbers
            .into_iter()
            .map(|value| Card { value, color })
            .collect();
        cards.shuffle(&mut rng);

        cards
    }

    /// Generate a CardSequence for a single player
    pub fn generate_player_sequence(color_1: Color, color_2: Color) -> Self {
        let mut rng = rand::thread_rng();
        let mut sequence = CardSequence::new();

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
        let sequences = vec![
            CardSequence::generate_player_sequence(CARD_COLORS[0], CARD_COLORS[1]),
            CardSequence::generate_player_sequence(CARD_COLORS[2], CARD_COLORS[3]),
        ];

        CardSequence::flatten_interleaved(sequences)
    }

    //  a Vec<CardSequence> into a CardSequence by interleaving each CardSequence in the Vec
    fn flatten_interleaved(vec: Vec<CardSequence>) -> CardSequence {
        let mut out = CardSequence::new();

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

pub fn generate_card_sequences(mut commands: Commands) {
    let sequence = CardSequence::generate_full_sequence();
    // TODO extend this for different number of players
    info!("generated card sequence {:?}", sequence);
    commands.insert_resource(sequence);
}
