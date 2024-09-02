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

    fn generate_color(color: Color) -> Self {
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

        Self { cards }
    }

    pub fn generate() -> Self {
        let mut all_sequences: Vec<CardSequence> = Vec::new();

        // Iterate over each color, generating a CardSequence and interleaving
        for color in CARD_COLORS.iter().cloned() {
            let single_color_cardsequence = Self::generate_color(color);
            all_sequences.push(single_color_cardsequence)
        }

        let out = Self::flatten_interleaved(all_sequences);
        info!("Generated CardSequence: {:?}", out);

        out
    }

    // Turn a Vec<CardSequence> into a CardSequence by interleaving each CardSequence in the Vec
    fn flatten_interleaved(vec: Vec<CardSequence>) -> CardSequence {
        let mut out = CardSequence::new();

        let max_len = vec.iter().map(|seq| seq.cards.len()).max().unwrap_or(0);

        for i in 0..max_len {
            for seq in vec.iter() {
                if let Some(card) = seq.cards.get(i) {
                    out.cards.push(card.clone());
                }
            }
        }

        out
    }
}

pub fn generate_player_card_sequences(mut commands: Commands) {
    let card_sequence = CardSequence::generate();
    commands.insert_resource(card_sequence);
}
