use bevy::prelude::*;

const CARDS_TO_WIN: i32 = 4;
pub const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const WIN_TEXT_FONT_SIZE: f32 = 100.0;
const WIN_TEXT_BOX_Z: f32 = 100.0;
const WIN_TEXT_BOX_X: f32 = WIN_TEXT_FONT_SIZE * 5.0;
const WIN_TEXT_BOX_Y: f32 = WIN_TEXT_FONT_SIZE * 3.0;

use crate::{
    asset_loader::AssetStore,
    board::bundle::GameState,
    camera::{CAMERA_OFFSET_X, CAMERA_OFFSET_Y},
    card::{
        bundle::Card,
        spawn::{CARD_TEXT_Z_OFFSET},
    },
};

use super::bundle::BOARD_SIZE;

#[derive(Resource)]
pub struct PlayerWinEntity(pub Option<Entity>);

pub fn check_wincondition(
    mut commands: Commands,
    board_state: Res<GameState>,
    asset_store: Res<AssetStore>,
    windows: Query<&Window>,
    mut player_win_entity: ResMut<PlayerWinEntity>,
) {
    if board_state.is_changed() {
        info!("Board state changed, checking wincondition");

        for (y, tiles) in board_state.board().iter().enumerate() {
            for (x, tile) in tiles.iter().enumerate() {
                if let Some(top_card) = tile.cards.last() {
                    // info!("checking neighbours of: {:?}", top_card);
                    if let Some(winning_card_streak) =
                        check_card_neighbours(*top_card, x, y, &board_state)
                    {
                        let entity = on_player_win(
                            winning_card_streak,
                            &mut commands,
                            &asset_store,
                            windows.single(),
                        );

                        info!("Inserting player win entity: {:?}", entity);
                        player_win_entity.0 = Some(entity);

                        // early return or else a player will always win at least 2 times, once for
                        // each end of the color streak.
                        // this will cause multiple calls to on_player_win, which is not expected
                        return;
                    }
                }
            }
        }
    }
}

fn on_player_win(
    winning_card_streak: Vec<Card>,
    commands: &mut Commands,
    asset_store: &Res<AssetStore>,
    window: &Window,
) -> Entity {
    let transform = Transform::from_xyz(
        window.width() / 2.0 + CAMERA_OFFSET_X,
        window.height() / 2.0 + CAMERA_OFFSET_Y,
        WIN_TEXT_BOX_Z,
    );

    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(WIN_TEXT_BOX_X, WIN_TEXT_BOX_Y)),
                ..Default::default()
            },
            transform,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "game over!".to_string(),
                    TextStyle {
                        font_size: WIN_TEXT_FONT_SIZE,
                        color: winning_card_streak.first().unwrap().color,
                        font: asset_store.font.clone(),
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, CARD_TEXT_Z_OFFSET),
                ..Default::default()
            });
        })
        .id();
    entity
}

// Checks each neighbouring card if there are CARDS_TO_WIN cards of the same color
fn check_card_neighbours(
    top_card: Card,
    x: usize,
    y: usize,
    board_state: &GameState,
) -> Option<Vec<Card>> {
    for (dx, dy) in NEIGHBOURS.iter() {
        'check_color_streak: for card_number in 1i32..=CARDS_TO_WIN {
            let mut current_color_streak = vec![top_card];

            let dx = dx * card_number;
            let dy = dy * card_number;
            if let Some(nx) = (x as i32).checked_add(dx) {
                if let Some(ny) = (y as i32).checked_add(dy) {
                    let board_size: i32 = BOARD_SIZE
                        .try_into()
                        .expect("BOARD_SIZE should always be a positive integer");
                    if nx >= 0 && nx < board_size && ny >= 0 && ny < board_size {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        if let Some(neighbouring_card) = board_state.get_tile(nx, ny).cards.last() {
                            if neighbouring_card.color != top_card.color {
                                break 'check_color_streak;
                            } else {
                                current_color_streak.push(*neighbouring_card);
                                // info!("color streak {:?}", current_color_streak);
                            }
                            if card_number == CARDS_TO_WIN - 1 {
                                return Some(current_color_streak);
                            }
                        } else {
                            break 'check_color_streak;
                        }
                    }
                }
            }
        }
    }

    None
    // check top_card neighbours, if same color
    // continue checking in that direction, if 4 consequitive WIN
}
