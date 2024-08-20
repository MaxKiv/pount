use bevy::prelude::*;

use crate::schedule::InGameSet;

const CARD_DIMENSIONS: Vec2 = Vec2::new(120.0, 120.0);

#[derive(Component, Debug)]
pub struct Card;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_card, despawn_cards)
                .chain()
                .in_set(InGameSet::MutateBoard),
        );
    }
}

fn spawn_card(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            let window = windows.single();

            // info!(
            //     "Window physical dimensions {:?} x {:?}",
            //     window.physical_width(),
            //     window.physical_height()
            // );
            // info!(
            //     "Window logical dimensions {:?} x {:?}",
            //     window.width(),
            //     window.height()
            // );

            // info!("Cursor at position {:?}", cursor_position);

            let card_position = cursor_to_screen_coordinates(cursor_position, &windows);
            // info!("Spawning card at position {:?}", card_position);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(CARD_DIMENSIONS), // 30x30 white square
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: card_position,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Card,
            ));
        }
    }
}

fn despawn_cards(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Card>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn cursor_to_screen_coordinates(cursor_position: Vec2, windows: &Query<&Window>) -> Vec3 {
    Vec3::new(
        cursor_position.x,
        windows.single().height() - cursor_position.y,
        0.0,
    )
}
