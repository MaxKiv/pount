use bevy::prelude::*;

/// [`Color`]s used in the game, must be multiple of 2
pub const CARD_COLORS: [Color; 4] = [
    Color::SALMON,
    Color::GOLD,
    Color::AQUAMARINE,
    Color::SEA_GREEN,
];

// Compile-time assertion to ensure array length is a multiple of 2
#[allow(dead_code)]
const fn assert_multiple_of_two(len: usize) {
    assert!(len % 2 == 0, "Array length must be a multiple of 2");
}
// Enforce the check by invoking the function in a const context
const _: () = assert_multiple_of_two(CARD_COLORS.len());

/// Marker struct for Cards
#[derive(Component, Debug)]
pub struct CardMarker;

/// Data component containing a card value and color
#[derive(Component, Clone, Copy)]
pub struct Card {
    pub value: i32,
    pub color: Color,
}

/// Bundle of components required to spawn and render a [`Card`]
#[derive(Bundle, Clone)]
pub struct CardBundle {
    pub card: Card,
    pub sprite: SpriteBundle,
}
