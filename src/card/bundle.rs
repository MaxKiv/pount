use ansi_term::Colour::RGB;
use bevy::prelude::*;

use crate::coordinates::TileCoordinates;

pub const CARD_DIMENSIONS: Vec2 = Vec2::new(100.0, 100.0);

#[derive(Component, Debug)]
pub struct CardMarker;

#[derive(Component)]
pub struct TilePosition(TileCoordinates);

impl TilePosition {
    pub fn new(tile_coordinates: TileCoordinates) -> Self {
        Self(tile_coordinates)
    }

    pub fn pos(&self) -> Vec3 {
        self.0.transform.translation
    }
}

#[derive(Clone, Component)]
pub struct Card {
    pub value: i32,
    pub color: Color,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::Rgba {
                red,
                green,
                blue,
                alpha,
            } => {
                // massage the Bevy::Color into a ansi_term::Colour
                let color = RGB(
                    (red * alpha * 255.0) as u8,
                    (green * alpha * 255.0) as u8,
                    (blue * alpha * 255.0) as u8,
                );
                write!(f, "{} {}", color.paint("█"), self.value)
            }
            _ => write!(f, "card with undefined color {}", self.value),
        }
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub position: TilePosition,
    pub sprite: SpriteBundle,
}

impl std::fmt::Debug for CardBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardBundle")
            .field("{:?}", &self.card)
            .finish()
    }
}
