use bevy::prelude::*;

use crate::board::bundle::{BOARD_SIZE, TILE_SIZE};

#[derive(Debug)]
pub struct LogicalCoordinates {
    transform: Transform,
}

impl LogicalCoordinates {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

impl LogicalCoordinates {
    pub fn from_cursor_position(cursor_position: Vec2) -> Self {
        Self {
            transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct ActuallyLogicalCoordinates {
    transform: Transform,
}

impl std::fmt::Debug for ActuallyLogicalCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameCoordinates")
            .field("translation", &self.transform.translation)
            .finish()
    }
}

impl ActuallyLogicalCoordinates {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn from_logical(mut value: LogicalCoordinates, window_height: f32) -> Self {
        let translation = value.transform.translation;
        value.transform.translation =
            Vec3::new(translation.x, window_height - translation.y, translation.z);
        ActuallyLogicalCoordinates::new(value.transform)
    }
}

#[derive(Clone)]
pub struct BoardCoordinates {
    pub transform: Transform,
}

impl BoardCoordinates {
    pub fn from_xyz(x: usize, y: usize, z: usize) -> Self {
        let x = x.clamp(0, BOARD_SIZE - 1);
        let y = y.clamp(0, BOARD_SIZE - 1);
        Self {
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
        }
    }

    pub fn as_xys(&self) -> (usize, usize, usize) {
        let translation = self.transform.translation;
        (
            translation.x as usize,
            translation.y as usize,
            translation.z as usize,
        )
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }
}

impl From<ActuallyLogicalCoordinates> for BoardCoordinates {
    fn from(value: ActuallyLogicalCoordinates) -> Self {
        let tile = value.transform().translation / Vec3::new(TILE_SIZE, TILE_SIZE, 1.0).round();
        Self::from_xyz(tile.x as usize, tile.y as usize, tile.z as usize)
    }
}

impl From<BoardCoordinates> for ActuallyLogicalCoordinates {
    fn from(value: BoardCoordinates) -> Self {
        let tile = value.transform().translation * Vec3::new(TILE_SIZE, TILE_SIZE, 1.0);

        Self {
            transform: Transform {
                translation: tile,
                ..default()
            },
        }
    }
}

impl std::fmt::Debug for BoardCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileCoordinates")
            .field("translation", &self.transform.translation)
            .finish()
    }
}
