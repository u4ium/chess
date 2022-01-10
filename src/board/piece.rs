use enum_map::Enum;

use std::{
    fmt::{self, Display, Formatter},
    ops::Not,
};

use crate::board::coordinates::*;
use RowIndex::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
use PieceType::*;

#[derive(Copy, Clone, Enum, Eq, PartialEq, Debug)]
pub enum Colour {
    Black,
    White,
}
use Colour::*;

use super::moves::CastlingAvailability;

impl Not for Colour {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl Default for Colour {
    fn default() -> Self {
        White
    }
}

impl Colour {
    pub fn home_rank(&self) -> RowIndex {
        match self {
            White => _1,
            Black => _8,
        }
    }

    pub fn home_pawn_rank(&self) -> RowIndex {
        match self {
            White => _2,
            Black => _7,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
    pub has_moved: bool,
}

pub trait Movable {
    fn moved(self, moved: bool) -> Self;
    fn guess_and_set_is_moved(
        &mut self,
        rank: RowIndex,
        file: ColumnIndex,
        castling_availability: &CastlingAvailability,
    );
}

impl Movable for Option<Piece> {
    fn moved(self, moved: bool) -> Self {
        self.and_then(|mut piece| {
            piece.has_moved = moved;
            Some(piece)
        })
    }

    fn guess_and_set_is_moved(
        &mut self,
        rank: RowIndex,
        file: ColumnIndex,
        castling_availability: &CastlingAvailability,
    ) {
        if let Some(piece) = self {
            match piece.piece_type {
                Rook => {
                    // assume a Rook has moved iff it is not available to castle (in principal)
                    piece.has_moved = !castling_availability[piece.colour][file];
                }
                Pawn => {
                    // assume a Pawn has moved iff it is not on its home rank
                    piece.has_moved = piece.colour.home_pawn_rank() != rank;
                }
                // assume other all pieces are moved
                _ => piece.has_moved = true,
            }
        }
    }
}

impl Piece {
    pub fn from_char(c: char) -> Result<Option<Self>, String> {
        let square = match c {
            'P' | '♙' => Piece::new(Pawn, White),
            'R' | '♖' => Piece::new(Rook, White),
            'N' | '♘' => Piece::new(Knight, White),
            'B' | '♗' => Piece::new(Bishop, White),
            'Q' | '♕' => Piece::new(Queen, White),
            'K' | '♔' => Piece::new(King, White),

            'p' | '♟' => Piece::new(Pawn, Black),
            'r' | '♜' => Piece::new(Rook, Black),
            'n' | '♞' => Piece::new(Knight, Black),
            'b' | '♝' => Piece::new(Bishop, Black),
            'q' | '♛' => Piece::new(Queen, Black),
            'k' | '♚' => Piece::new(King, Black),

            '_' | ' ' | '◻' | '◼' => return Ok(None),

            _ => return Err(format!("Invalid character for square {}", c)),
        };
        Ok(Some(square))
    }

    pub fn new(piece_type: PieceType, colour: Colour) -> Self {
        Self {
            piece_type,
            colour,
            has_moved: false,
        }
    }

    pub fn get_value(&self) -> f64 {
        let value = match self.piece_type {
            Pawn => 0.01,
            Knight => 0.03,
            Bishop => 0.04,
            Rook => 0.05,
            Queen => 0.09,
            King => 0.0,
        };
        match self.colour {
            White => value,
            Black => -value,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match (self.colour, self.piece_type) {
                (White, Pawn) => '♙',
                (White, Rook) => '♖',
                (White, Knight) => '♘',
                (White, Bishop) => '♗',
                (White, Queen) => '♕',
                (White, King) => '♔',

                (Black, Pawn) => '♟',
                (Black, Rook) => '♜',
                (Black, Knight) => '♞',
                (Black, Bishop) => '♝',
                (Black, Queen) => '♛',
                (Black, King) => '♚',
            }
        )
    }
}
