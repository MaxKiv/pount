use bevy::{prelude::*, reflect::Array};

use crate::coordinates::{DiscretisedGameCoordinates, GameCoordinates, LogicalCoordinates};

const CARD_DIMENSIONS: Vec2 = Vec2::new(135.0, 135.0);

const CARD_COLORS: [Color; 4] = [
    Color::SALMON,
    Color::AQUAMARINE,
    Color::GOLD,
    Color::SEA_GREEN,
];

// const GRID_Y: [f32; 10] = [
//     0.0, 80.0, 160.0, 240.0, 320.0, 400.0, 480.0, 560.0, 640.0, 720.0,
// ];

#[derive(Component, Debug)]
pub struct CardMarker;

#[derive(Component, Debug)]
pub struct IntegerValue(i32);

#[derive(Component, Debug)]
pub struct ColorComponent(Color);

impl ColorComponent {
    pub fn color(&self) -> Color {
        self.0
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub value: IntegerValue,
    pub color: ColorComponent,
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

impl CardBundle {
    pub fn new(value: i32, color: Color, at_position: DiscretisedGameCoordinates) -> Self {
        Self {
            value: IntegerValue(value),
            color: ColorComponent(color),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(CARD_DIMENSIONS), // 30x30 white square
                    ..Default::default()
                },
                transform: at_position.transform,
                ..Default::default()
            },
        }
    }
}

#[derive(Resource)]
pub struct ColorIndex {
    index: usize,
}

impl ColorIndex {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

pub fn spawn_card(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut color_index: ResMut<ColorIndex>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            // info!("Cursor at position {:?}", cursor_position);

            let logical_coordinates = LogicalCoordinates::from_cursor_position(cursor_position);
            let game_coordinates =
                GameCoordinates::from_logical(logical_coordinates, windows.single().height());
            info!("logical coordinates: {:?}", game_coordinates);

            // let card_position = get_card_grid_position(card_position);
            let discrete_coordinates: DiscretisedGameCoordinates = game_coordinates.into();

            let card_position = discrete_coordinates.transform.translation;
            info!("spawning card at: {:?}", card_position);

            let color = next_card_color(&mut color_index);
            commands.spawn((CardBundle::new(0, color, discrete_coordinates), CardMarker));
        }
    }
}

pub fn despawn_cards(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<CardMarker>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn next_card_color(color_index: &mut ColorIndex) -> Color {
    let color = CARD_COLORS[color_index.index];
    color_index.index = (color_index.index + 1) % CARD_COLORS.len();
    color
}
