use bevy::prelude::*;

use crate::card::bundle::Card;

pub const TILE_SIZE: f32 = 125.0;
pub const BOARD_SIZE: usize = 10;

type Board = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// TODO remove the pub from Board, to make the GameBoard data structure opaque
#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    // TODO it does not seem very clean to pass a bool everywhere just to see if this was the first
    // move
    pub empty: bool,
}

impl GameState {
    pub fn reset() -> Self {
        let empty_board: GameState = GameState {
            board: core::array::from_fn(|_| core::array::from_fn(|_| Tile { cards: Vec::new() })),
            empty: true,
        };
        empty_board
    }

    pub fn clear(&mut self) {
        for tiles in self.board.iter_mut() {
            for tile in tiles.iter_mut() {
                tile.cards.clear();
            }
        }
        self.empty = true;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.board[y][x]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        // HACK: this assumption will not always hold
        self.empty = false;
        &mut self.board[y][x]
    }

    // TODO remove this to make GameBoard data structure opaque
    pub fn board(&self) -> &Board {
        &self.board
    }
}

impl std::fmt::Debug for GameState {
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
    let empty_board = GameState::reset();
    commands.insert_resource(empty_board);
}
