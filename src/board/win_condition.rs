use bevy::prelude::*;

const CARDS_TO_WIN: i32 = 4;
const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

use crate::{asset_loader::AssetStore, board::bundle::GameBoard, card::bundle::Card};

use super::bundle::BOARD_SIZE;

#[derive(Resource, Debug)]
pub struct StateChanged(pub bool);

pub fn check_wincondition(
    mut commands: Commands,
    board_state: Res<GameBoard>,
    mut board_state_changed: ResMut<StateChanged>,
    asset_store: Res<AssetStore>,
) {
    info!("Board state changed, checking wincondition");
    if board_state_changed.0 {
        board_state_changed.0 = false;

        for (y, tiles) in board_state.board().iter().enumerate() {
            for (x, tile) in tiles.iter().enumerate() {
                if let Some(top_card) = tile.cards.first() {
                    if let Some(winning_color) =
                        check_card_neighbours(*top_card, x, y, &board_state)
                    {
                        commands.spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("{:?} won!", winning_color),
                                TextStyle {
                                    font_size: 250.0,
                                    color: Color::PINK,
                                    font: asset_store.font.clone(),
                                },
                            ),
                            // Overlay the text on the card by setting its Z value
                            transform: Transform::from_xyz(800.0, 800.0, 100.0),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }
}

// Checks each neighbouring card if there are CARDS_TO_WIN cards of the same color
fn check_card_neighbours(
    top_card: Card,
    x: usize,
    y: usize,
    board_state: &GameBoard,
) -> Option<Color> {
    'check_neighbor: for (dx, dy) in NEIGHBOURS.iter() {
        for card_number in 0i32..CARDS_TO_WIN {
            if let Some(nx) = (x as i32)
                .checked_add(*dx)
                .and_then(|nx| nx.checked_add(card_number))
            {
                if let Some(ny) = (y as i32)
                    .checked_add(*dy)
                    .and_then(|ny| ny.checked_add(card_number))
                {
                    let board_size: i32 = BOARD_SIZE
                        .try_into()
                        .expect("BOARD_SIZE should always be a positive integer");
                    if nx >= 0 && nx < board_size && ny >= 0 && ny < board_size {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        // Attempt to get the neighbouring card
                        if let Some(neighbouring_card) = board_state.get_tile(nx, ny).cards.first()
                        {
                            if neighbouring_card.color != top_card.color {
                                break 'check_neighbor;
                            } else if card_number == CARDS_TO_WIN - 1 {
                                return Some(top_card.color);
                            }
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
