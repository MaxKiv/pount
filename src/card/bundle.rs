use bevy::prelude::*;

use crate::coordinates::TileCoordinates;

pub const CARD_DIMENSIONS: Vec2 = Vec2::new(100.0, 100.0);

#[derive(Component, Debug)]
pub struct CardMarker;

#[derive(Component, Debug)]
pub struct Weight(pub i32);

impl Weight {
    pub fn weight(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
pub struct ColorComponent(pub Color);

impl ColorComponent {
    pub fn color(&self) -> Color {
        self.0
    }
}

impl std::fmt::Debug for ColorComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Color::SALMON => write!(f, "Salmon"),
            Color::AQUAMARINE => write!(f, "Aquamarine"),
            Color::GOLD => write!(f, "Gold"),
            Color::SEA_GREEN => write!(f, "Green"),
            _ => write!(f, "undefined color"),
        }
    }
}

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

#[derive(Bundle)]
pub struct CardBundle {
    pub value: Weight,
    pub color: ColorComponent,
    pub position: TilePosition,
    pub sprite: SpriteBundle,
}

impl std::fmt::Debug for CardBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardBundle")
            .field("value", &self.value)
            .field("color", &self.color)
            .finish()
    }
}
