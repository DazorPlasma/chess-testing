#![deny(unsafe_code)]

use std::sync::{Arc, RwLock, Weak};

use super::Board;

const VALID_POSITION_RANGE: std::ops::RangeInclusive<i8> = 0..=7;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "white"),
            Self::Black => write!(f, "black"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kind {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Self::King => "king",
            Self::Queen => "queen",
            Self::Bishop => "bishop",
            Self::Knight => "knight",
            Self::Pawn => "pawn",
            Self::Rook => "rook",
        };
        write!(f, "{txt}")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// Position (0, 0) is the bottom-left corner
pub struct Position {
    x: i8,
    y: i8,
}
#[derive(Debug, Clone)]
pub struct Piece {
    color: Color,
    kind: Kind,
    position: Position,
    #[allow(unused)]
    board: Weak<RwLock<Board>>,
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.kind == other.kind && self.position == other.position
    }
}
impl Piece {
    pub fn new(kind: Kind, color: Color, position: Position, board: &Arc<RwLock<Board>>) -> Piece {
        Piece {
            kind,
            color,
            position,
            board: Arc::downgrade(board),
        }
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_kind(&self) -> Kind {
        self.kind
    }
    pub fn get_position(&self) -> Position {
        self.position
    }
    #[allow(unused)]
    fn get_moves(&self) -> Vec<Position> {
        let mut moves = Vec::with_capacity(32);
        let current_position = self.position;
        match self.kind {
            Kind::King => {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let new_position =
                            Position::new(current_position.x + i, current_position.y + j);
                        moves.push(new_position);
                    }
                }
            }
            Kind::Queen => todo!(),
            Kind::Knight => todo!(),
            Kind::Bishop => todo!(),
            Kind::Rook => todo!(),
            Kind::Pawn => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
        }

        moves.into_iter().filter(|&p| p.is_valid()).collect()
    }
}

impl Position {
    pub fn new(x: i8, y: i8) -> Position {
        let pos = Position { x, y };
        if !pos.is_valid() {
            panic!("Tried to create invalid position: ({}, {})", x, y);
        }
        pos
    }
    fn is_valid(&self) -> bool {
        VALID_POSITION_RANGE.contains(&self.x) && VALID_POSITION_RANGE.contains(&self.y)
    }
    pub fn x(&self) -> i8 {
        self.x
    }
    pub fn y(&self) -> i8 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn piece_creation() {
        let pawn = Piece::new(
            Kind::Pawn,
            Color::Black,
            Position::new(2, 3),
            &Board::new_empty(),
        );
        assert_eq!(pawn.kind, Kind::Pawn);
        assert_eq!(pawn.color, Color::Black);
        assert_eq!(pawn.position, Position { x: 2, y: 3 });

        let king = Piece::new(
            Kind::King,
            Color::White,
            Position::new(0, 7),
            &Board::new_empty(),
        );
        assert_eq!(king.kind, Kind::King);
        assert_eq!(king.color, Color::White);
        assert_eq!(king.position, Position { x: 0, y: 7 });
    }

    #[test]
    #[should_panic]
    fn outside_board_creation() {
        Piece::new(
            Kind::Pawn,
            Color::White,
            Position::new(8, 1),
            &Board::new_empty(),
        );
    }
}
