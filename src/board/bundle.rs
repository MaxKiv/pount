use bevy::prelude::*;

use crate::card::bundle::Card;

pub const TILE_SIZE: f32 = 125.0;
pub const BOARD_SIZE: usize = 10;

type Board = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// TODO remove the pub from Board, to make the GameBoard data structure opaque
#[derive(Resource)]
pub struct GameBoard(pub Board);

impl GameBoard {
    pub fn reset() -> Self {
        let empty_board: GameBoard = GameBoard(core::array::from_fn(|_| {
            core::array::from_fn(|_| Tile { cards: Vec::new() })
        }));
        empty_board
    }

    pub fn clear(&mut self) {
        for tiles in self.0.iter_mut() {
            for tile in tiles.iter_mut() {
                tile.cards.clear();
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.0[y][x]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.0[y][x]
    }

    // TODO remove this to make GameBoard data structure opaque
    pub fn board(&self) -> &Board {
        &self.0
    }
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, tiles) in self.board().iter().enumerate() {
            for (x, tile) in tiles.iter().enumerate() {
                if let Some(top_card) = tile.cards.last() {
                    write!(f, "({},{}) {:?} ", x, y, top_card).unwrap();
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Tile {
    pub cards: Vec<Card>,
}

// System to initialize the game board
pub fn setup_board(mut commands: Commands) {
    let empty_board = GameBoard::reset();
    commands.insert_resource(empty_board);
}
