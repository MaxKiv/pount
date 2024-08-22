use bevy::prelude::*;
use rand::prelude::SliceRandom;

const NUMBERS: std::ops::RangeInclusive<i32> = 1..=9;

#[derive(Debug)]
pub struct CardSequence {
    numbers: Vec<i32>,
}

impl CardSequence {
    fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<i32> = NUMBERS.clone().chain(NUMBERS).collect();
        numbers.shuffle(&mut rng);

        info!("Generated CardSequence: {:?}", numbers);
        Self { numbers }
    }
}

pub fn generate_player_card_sequences() {
    CardSequence::generate();
}
