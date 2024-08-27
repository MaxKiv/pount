use std::default;

use bevy::prelude::*;

use crate::{
    asset_loader::AssetStore,
    coordinates::{ActuallyLogicalCoordinates, LogicalCoordinates, TileCoordinates},
};

const CARD_DIMENSIONS: Vec2 = Vec2::new(100.0, 100.0);

#[derive(Component, Debug)]
pub struct CardMarker;

#[derive(Component, Debug)]
pub struct Weight(i32);

impl Weight {
    pub fn weight(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
pub struct ColorComponent(Color);

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
pub struct Position(TileCoordinates);

impl Position {
    pub fn pos(&self) -> Vec3 {
        self.0.transform.translation
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub value: Weight,
    pub color: ColorComponent,
    pub position: Position,
    sprite: SpriteBundle,
}

impl std::fmt::Debug for CardBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardBundle")
            .field("value", &self.value)
            .field("color", &self.color)
            .finish()
    }
}

impl CardBundle {
    pub fn new(value: i32, color: Color, at_position: TileCoordinates) -> Self {
        let spawning_coordinates: ActuallyLogicalCoordinates = at_position.clone().into();

        Self {
            value: Weight(value),
            color: ColorComponent(color),
            position: Position(at_position),

            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(CARD_DIMENSIONS),
                    ..Default::default()
                },
                transform: spawning_coordinates.transform(),
                ..Default::default()
            },
        }
    }
}
