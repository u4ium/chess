use enum_map::Enum;

use std::{
    fmt::{self, Display, Formatter},
    ops::Not,
};

use crate::board::{coordinates::*, grid::*, MoveRecord};
use ColumnIndex::*;
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

    pub fn check_move(
        &self,
        board: &Board,
        m: Move,
        by: Colour,
        en_passant_destination: Option<Coordinate>,
    ) -> Result<MoveRecord, String> {
        if self.colour != by {
            return Err(String::from("Cannot move opponent's piece"));
        }

        if m.from == m.to {
            return Err(String::from("Must move piece"));
        }

        let destination_square = board[m.to.row][m.to.column];
        match destination_square {
            Some(t) if t.colour == by => {
                return Err(String::from("Cannot take own piece"));
            }
            _ => {}
        }

        let first_move = !self.has_moved;
        let return_move_record = || match destination_square {
            Some(taken) => Ok(MoveRecord::TakeMove {
                m,
                taken,
                taken_from: m.to,
                first_move,
            }),
            None => Ok(MoveRecord::SimpleMove { m, first_move }),
        };
        let check_path_and_return_move_record = |m| match has_no_pieces_between(board, &m) {
            Ok(()) => return_move_record(),
            Err((p, c)) => Err(format!(
                "Cannot move here: blocked at ({:?}, {:?}), by {:?}",
                c.row, c.column, p
            )),
        };

        let d_row = (m.to.row as i8) - (m.from.row as i8);
        let d_column = (m.to.column as i8) - (m.from.column as i8);

        let end_row = if by == White { _8 } else { _1 };
        let row_increment = if by == White { -1 } else { 1 };
        match self.piece_type {
            Pawn => match d_column {
                0 => match destination_square {
                    None => match d_row * if by == White { -1 } else { 1 } {
                        1 => {
                            if m.to.row == end_row {
                                Ok(MoveRecord::PawnPromotion {
                                    m,
                                    to: Queen, // TODO
                                    taken: None,
                                })
                            } else {
                                return_move_record()
                            }
                        }
                        2 => {
                            let in_between = if by == White { _3 } else { _6 };
                            if !self.has_moved {
                                if board[in_between][m.from.column].is_none() {
                                    return_move_record()
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
                1 | -1 => match d_row * row_increment {
                    0 => Err(String::from("Cannot move Pawn horizontally")),
                    1 => match destination_square {
                        Some(taken_piece) => {
                            if m.to.row == end_row {
                                Ok(MoveRecord::PawnPromotion {
                                    m,
                                    to: Queen, // TODO
                                    taken: Some(taken_piece),
                                })
                            } else {
                                return_move_record()
                            }
                        }
                        None if en_passant_destination == Some(m.to) => {
                            let taken_from = Coordinate {
                                column: m.to.column,
                                row: RowIndex::from((m.to.row as i8 - row_increment) as usize),
                            };
                            let taken = board[taken_from.row][taken_from.column].unwrap(); // SAFE
                            Ok(MoveRecord::TakeMove {
                                m,
                                taken,
                                taken_from,
                                first_move: !self.has_moved,
                            })
                        }
                        None => Err(String::from("Cannot move Pawn diagonally unless taking")),
                    },
                    2..=6 => Err(String::from("Cannot move Pawn here: too far")),
                    _ => Err(String::from("Cannot move Pawn here: must move forwards")),
                },
                _ => match d_row {
                    0 => Err(String::from("Cannot move Pawn horizontally")),
                    _ => Err(String::from("Cannot move Pawn this way")),
                },
            },
            Rook => match (d_row, d_column) {
                (_, 0) | (0, _) => check_path_and_return_move_record(m),
                _ => Err(String::from("Cannot move Rook here: not a straight line")),
            },
            Knight => match (i8::abs(d_row), i8::abs(d_column)) {
                (1, 2) => return_move_record(),
                (2, 1) => return_move_record(),
                _ => Err(String::from("Cannot move Knight here: not in L pattern")),
            },
            Bishop => match (i8::abs(d_row), i8::abs(d_column)) {
                (r, c) if r == c => check_path_and_return_move_record(m),
                _ => Err(String::from("Cannot move Bishop here: not a diagonal line")),
            },
            Queen => match (i8::abs(d_row), i8::abs(d_column)) {
                (_, 0) | (0, _) => check_path_and_return_move_record(m),
                (r, c) if r == c => check_path_and_return_move_record(m),
                _ => Err(String::from("Cannot move Queen here: not in a line")),
            },
            King => match (d_row, d_column) {
                (-1 | 1, -1 | 1) => return_move_record(), // Diagonal
                (-1 | 1, 0) => return_move_record(),      // Vertical
                (0, -1 | 1) => return_move_record(),      // Horizontal
                // TODO (Fischer Random support): use Move enum instead of difference by 2
                (0, direction @ (-2 | 2)) => {
                    // Castle
                    // check king has not moved
                    if self.has_moved {
                        return Err(String::from("Cannot castle: King has been moved"));
                    }
                    // find target rook
                    let rook_coordinates = Coordinate {
                        row: m.from.row,
                        column: if direction == 2 { H } else { A },
                    };
                    let rook = match board[rook_coordinates.row][rook_coordinates.column] {
                        Some(piece) => piece,
                        None => {
                            return Err(String::from("Cannot castle: no Rook found"));
                        }
                    };
                    // check target rook has not moved
                    if rook.has_moved {
                        return Err(String::from(
                            "Cannot castle this way: target Rook has been moved",
                        ));
                    }
                    // check that castling does not take a piece (TODO: redundant, except for Fischer?)
                    if let Some(_) = destination_square {
                        return Err(String::from(
                            "Cannot castle: cannot take piece during castle",
                        ));
                    }
                    // check_path king -> rook
                    match has_no_pieces_between(board, &m) {
                        Ok(()) => {
                            let rook_move = Move {
                                from: rook_coordinates,
                                to: Coordinate {
                                    row: rook_coordinates.row,
                                    column: if direction == 2 { F } else { D },
                                },
                            };
                            Ok(MoveRecord::CastleMove {
                                king_move: m,
                                rook_move,
                            })
                        }
                        Err((p, c)) => Err(format!(
                            "Cannot move here: blocked at ({:?}, {:?}), by {:?}",
                            c.row, c.column, p
                        )),
                    }
                    // TODO: check in-through-out-of-check
                }
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
