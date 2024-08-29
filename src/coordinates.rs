use bevy::prelude::*;

const TILESIZE: f32 = 125.0;

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
pub struct TileCoordinates {
    pub transform: Transform,
}

impl TileCoordinates {
    pub fn transform(&self) -> Transform {
        self.transform
    }
}

impl From<ActuallyLogicalCoordinates> for TileCoordinates {
    fn from(value: ActuallyLogicalCoordinates) -> Self {
        // let discrete = get_card_grid_position(value);
        let tile = value.transform().translation / Vec3::new(TILESIZE, TILESIZE, 1.0).round();
        let tile = tile.round();

        Self {
            transform: Transform {
                translation: tile,
                ..default()
            },
        }
    }
}

impl From<TileCoordinates> for ActuallyLogicalCoordinates {
    fn from(value: TileCoordinates) -> Self {
        // let discrete = get_card_grid_position(value);
        let tile = value.transform().translation * Vec3::new(TILESIZE, TILESIZE, 1.0);

        Self {
            transform: Transform {
                translation: tile,
                ..default()
            },
        }
    }
}

impl std::fmt::Debug for TileCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileCoordinates")
            .field("translation", &self.transform.translation)
            .finish()
    }
}
