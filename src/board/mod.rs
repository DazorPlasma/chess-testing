#![allow(unused)]
mod piece;

use bevy::{ecs::system::Resource, utils::HashMap};
use piece::*;
pub use piece::{Color, Kind, Position};
use std::sync::{Arc, RwLock, Weak};

#[derive(Debug, Resource)]
pub struct Board {
    pieces: Vec<Piece>,
    pieces_map: HashMap<Position, usize>,
}

fn add_white_pieces(board: Arc<RwLock<Board>>) {
    use Position as Pos;
    let pieces = &mut board.write().unwrap().pieces;
    let white = Color::White;

    for i in 0..=7 {
        let pawn = Piece::new(Kind::Pawn, white, Pos::new(1, i), &board);
        pieces.push(pawn);
    }

    let left_rook = Piece::new(Kind::Rook, white, Pos::new(0, 0), &board);
    let right_rook = Piece::new(Kind::Rook, white, Pos::new(0, 7), &board);
    let left_knight = Piece::new(Kind::Knight, white, Pos::new(0, 1), &board);
    let right_knight = Piece::new(Kind::Knight, white, Pos::new(0, 6), &board);
    let left_bishop = Piece::new(Kind::Bishop, white, Pos::new(0, 2), &board);
    let right_bishop = Piece::new(Kind::Bishop, white, Pos::new(0, 5), &board);
    let queen = Piece::new(Kind::Queen, white, Pos::new(0, 3), &board);
    let king = Piece::new(Kind::King, white, Pos::new(0, 4), &board);

    pieces.push(left_rook);
    pieces.push(right_rook);
    pieces.push(left_knight);
    pieces.push(right_knight);
    pieces.push(left_bishop);
    pieces.push(right_bishop);
    pieces.push(queen);
    pieces.push(king);
}
fn add_black_pieces(board: Arc<RwLock<Board>>) {
    use Position as Pos;
    let pieces = &mut board.write().unwrap().pieces;
    let black = Color::Black;

    for i in 0..=7 {
        let pawn = Piece::new(Kind::Pawn, black, Pos::new(6, i), &board);
        pieces.push(pawn);
    }

    let left_rook = Piece::new(Kind::Rook, black, Pos::new(7, 0), &board);
    let right_rook = Piece::new(Kind::Rook, black, Pos::new(7, 7), &board);
    let left_knight = Piece::new(Kind::Knight, black, Pos::new(7, 1), &board);
    let right_knight = Piece::new(Kind::Knight, black, Pos::new(7, 6), &board);
    let left_bishop = Piece::new(Kind::Bishop, black, Pos::new(7, 2), &board);
    let right_bishop = Piece::new(Kind::Bishop, black, Pos::new(7, 5), &board);
    let queen = Piece::new(Kind::Queen, black, Pos::new(7, 4), &board);
    let king = Piece::new(Kind::King, black, Pos::new(7, 3), &board);

    pieces.push(left_rook);
    pieces.push(right_rook);
    pieces.push(left_knight);
    pieces.push(right_knight);
    pieces.push(left_bishop);
    pieces.push(right_bishop);
    pieces.push(queen);
    pieces.push(king);
}
fn add_default_pieces(board: &Arc<RwLock<Board>>) {
    add_white_pieces(Arc::clone(board));
    add_black_pieces(Arc::clone(board));
}

impl Board {
    fn new_empty() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Board {
            pieces: Vec::with_capacity(32),
            pieces_map: HashMap::new(),
        }))
    }
    pub fn new() -> Arc<RwLock<Self>> {
        let mut board = Board::new_empty();
        add_default_pieces(&board);
        let mut board_lock = board.write().unwrap();
        let piece_positions: Vec<Position> =
            board_lock.pieces.iter().map(|x| x.get_position()).collect();
        for (index, piece) in piece_positions.iter().enumerate() {
            board_lock.pieces_map.insert(*piece, index);
        }
        drop(board_lock);
        board
    }
    pub fn get_pieces(&self) -> &[Piece] {
        &self.pieces
    }
}
