use enum_map::Enum;

use std::fmt::{self, Display, Formatter};

use crate::board::{coordinates::*, grid::*};
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

pub fn other_player(player: Colour) -> Colour {
    match player {
        White => Black,
        Black => White,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
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

            '_' | ' ' => return Ok(None),
            _ => return Err(format!("Invalid character for square {}", c)),
        };
        Ok(Some(square))
    }

    pub fn new(piece_type: PieceType, colour: Colour) -> Self {
        Self { piece_type, colour }
    }

    pub fn check_move(&self, board: &Board, m: &Move, by: Colour) -> Result<Coordinate, String> {
        if self.colour != by {
            return Err(String::from("Cannot move opponent's piece"));
        }
        if m.from == m.to {
            return Err(String::from("Must move piece"));
        }
        let taken = board[m.to.row][m.to.column];
        match taken {
            Some(t) if t.colour == by => {
                return Err(String::from("Cannot take own piece"));
            }
            _ => {}
        }
        let destination = Coordinate { ..m.to };
        let d_row = (m.to.row as i8) - (m.from.row as i8);
        let d_column = (m.to.column as i8) - (m.from.column as i8);
        let check_path = || -> Result<Coordinate, String> {
            match has_no_pieces_between(board, m) {
                Ok(()) => Ok(destination),
                Err((p, c)) => Err(format!(
                    "Cannot move here: blocked at ({:?}, {:?}), by {:?}",
                    c.row, c.column, p
                )),
            }
        };
        match self.piece_type {
            Pawn => match d_column {
                0 => match taken {
                    None => match d_row * if by == White { -1 } else { 1 } {
                        1 => Ok(destination),
                        2 => {
                            let start_row = if by == White { _2 } else { _7 };
                            let in_between = if by == White { _3 } else { _6 };
                            if m.from.row == start_row {
                                if board[in_between][m.from.column].is_none() {
                                    Ok(destination)
                                } else {
                                    Err(String::from("Cannot jump with Pawn"))
                                }
                            } else {
                                Err(String::from("Cannot double move Pawn unless on start row"))
                            }
                        }
                        3..=7 => Err(String::from("Cannot move Pawn more than two squares")),
                        _ => Err(String::from("Cannot move Pawn backwards")),
                    },
                    Some(_) => Err(String::from("Cannot take with Pawn unless diagonally")),
                },
                1 | -1 => match d_row * if by == White { -1 } else { 1 } {
                    0 => Err(String::from("Cannot move Pawn horizontally")),
                    1 => match taken {
                        Some(_) => Ok(destination),
                        // TODO: En-Passant
                        None => Err(String::from("Cannot move Pawn diagonally unless taking")),
                    },
                    2..=7 => Err(String::from("Cannot move Pawn here: too far")),
                    _ => Err(String::from("Cannot move Pawn here: must move forwards")),
                },
                _ => Err(String::from("Cannot move Pawn horizontally")),
            },
            Rook => match (d_row, d_column) {
                (_, 0) | (0, _) => check_path(),
                _ => Err(String::from("Cannot move Rook here: not a straight line")),
            },
            Knight => match (i8::abs(d_row), i8::abs(d_column)) {
                (1, 2) => Ok(destination),
                (2, 1) => Ok(destination),
                _ => Err(String::from("Cannot move Knight here: not in L pattern")),
            },
            Bishop => match (i8::abs(d_row), i8::abs(d_column)) {
                (r, c) if r == c => check_path(),
                _ => Err(String::from("Cannot move Bishop here: not a diagonal line")),
            },
            Queen => match (i8::abs(d_row), i8::abs(d_column)) {
                (_, 0) | (0, _) => check_path(),
                (r, c) if r == c => check_path(),
                _ => Err(String::from("Cannot move Queen here: not in a line")),
            },
            King => match (d_row, d_column) {
                (-1 | 1, -1 | 1 | 0) => Ok(destination),
                (0, -1 | 1) => Ok(destination),
                // TODO: Castle
                _ => Err(String::from("Cannot move King more than one square")),
            },
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
