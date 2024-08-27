use bevy::{prelude::*, reflect::Array};

use crate::{
    asset_loader::AssetStore,
    card::bundle::{CardBundle, CardMarker},
    coordinates::{ActuallyLogicalCoordinates, LogicalCoordinates, TileCoordinates},
};

const CARD_COLORS: [Color; 4] = [
    Color::SALMON,
    Color::AQUAMARINE,
    Color::GOLD,
    Color::SEA_GREEN,
];

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
    asset_store: Res<AssetStore>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            // info!("Cursor at position {:?}", cursor_position);

            let logical_coordinates = LogicalCoordinates::from_cursor_position(cursor_position);
            let game_coordinates = ActuallyLogicalCoordinates::from_logical(
                logical_coordinates,
                windows.single().height(),
            );
            // info!("logical coordinates: {:?}", game_coordinates);

            // let card_position = get_card_grid_position(card_position);
            let tile_coordinates: TileCoordinates = game_coordinates.into();

            let card_position = tile_coordinates.transform.translation;
            info!("spawning card at: {:?}", card_position);

            let color = next_card_color(&mut color_index);
            let card_entity = commands
                .spawn((
                    CardBundle::new(0, color, tile_coordinates.clone()),
                    CardMarker,
                ))
                .id();

            // Spawn the text entity as a child of the card entity
            commands.entity(card_entity).with_children(|parent| {
                let mut text_coordinates = tile_coordinates.transform();
                text_coordinates.translation += Vec3::new(0.0, 0.0, 1.0);

                parent.spawn(TextBundle {
                    text: Text::from_section(
                        0.to_string(),
                        TextStyle {
                            font: asset_store.font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    // The transform to overlay the text on the sprite
                    transform: text_coordinates,
                    ..Default::default()
                });
            });
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
