use bevy::prelude::*;

use crate::card::bundle::Card;

pub const TILE_SIZE: f32 = 125.0;
pub const BOARD_SIZE: usize = 10;

type Board = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// TODO remove the pub from Board, to make the GameBoard data structure opaque
#[derive(Resource, Debug)]
pub struct GameBoard(pub Board);

impl GameBoard {
    pub fn initialize() -> Self {
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

    // pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    //     &self.0[y][x]
    // }

    // TODO remove this to make GameBoard data structure opaque
    pub fn board(&self) -> &Board {
        &self.0
    }
}

#[derive(Debug)]
pub struct Tile {
    pub cards: Vec<Card>,
}

// System to initialize the game board
pub fn setup_board(mut commands: Commands) {
    let empty_board = GameBoard::initialize();

    info!("spawned GameBoard resource: {:?}", empty_board);
    commands.insert_resource(empty_board);
}
