use std::f32::consts::PI;

use bevy::{prelude::*, transform};

const GRID: [f32; 10] = [
    0.0, 150.0, 300.0, 450.0, 600.0, 750.0, 900.0, 1050.0, 1200.0, 1350.0,
];

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

#[derive(Debug, Clone)]
pub struct GameCoordinates {
    transform: Transform,
}

impl GameCoordinates {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }
}

impl GameCoordinates {
    pub fn from_logical(mut value: LogicalCoordinates, window_height: f32) -> Self {
        let translation = value.transform.translation;
        value.transform.translation =
            Vec3::new(translation.x, window_height - translation.y, translation.z);
        GameCoordinates::new(value.transform)
    }
}

#[derive(Debug)]
pub struct DiscretisedGameCoordinates {
    pub transform: Transform,
}

impl From<GameCoordinates> for DiscretisedGameCoordinates {
    fn from(value: GameCoordinates) -> Self {
        Self {
            transform: Transform {
                translation: get_card_grid_position(value),
                ..default()
            },
        }
    }
}

fn get_card_grid_position(card_position: GameCoordinates) -> Vec3 {
    Vec3::new(
        snap_to(card_position.transform().translation.x, &GRID),
        snap_to(card_position.transform().translation.y, &GRID),
        card_position.transform().translation.z,
    )
}

fn snap_to(value: f32, grid: &[f32; 10]) -> f32 {
    // info!("snapping {:?} to {:?}", value, grid);
    let mut last_difference = f32::MAX;
    let mut output = 0.0;

    for gridpoint in grid {
        let difference_to_gridpoint = f32::abs(value - gridpoint);
        if difference_to_gridpoint < last_difference {
            last_difference = difference_to_gridpoint;
            output = *gridpoint;
        } else {
            break;
        }
    }

    // info!("result of snap: {:?}", output);
    output
}
