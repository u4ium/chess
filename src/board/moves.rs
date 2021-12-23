use std::ops::{Deref, DerefMut};

use super::{
    coordinates::{ColumnIndex, Coordinate, Move, RowIndex},
    grid::Board,
    piece::{Colour, Piece, PieceType::Pawn},
};
use enum_map::EnumMap;

#[derive(Debug, PartialEq)]
pub struct CastlingAvailability(pub EnumMap<Colour, EnumMap<ColumnIndex, bool>>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MoveRecord {
    SimpleMove {
        /// the move that mas made
        m: Move,
        /// true iff this is the first move of this piece
        first_move: bool,
    },
    TakeMove {
        /// the move that mas made
        m: Move,
        /// the piece that was taken
        taken: Piece,
        /// for en-passant: may not be the same as m.to
        taken_from: Coordinate,
        /// true iff this is the first move of this piece
        first_move: bool,
    },
    CastleMove(Move, Move),
}

pub enum CastleDirection {
    UpFile,
    DownFile,
}

// TODO: use for public API: check_move
pub enum MoveSignal {
    Retire,
    ForceDraw,
    RequestDraw,
    Castle(CastleDirection),
    Make(Move),
}

#[derive(Debug, PartialEq)]
pub struct MoveRecords(Vec<MoveRecord>);

impl MoveRecords {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn get_en_passant_availability(&self, board: &Board) -> Option<Coordinate> {
        if let Some(move_record) = self.0.last() {
            match move_record {
                MoveRecord::SimpleMove { m, .. } => {
                    let piece = board[m.to.row][m.to.column].expect(
                        "There should be a piece at the destination of the last SimpleMove",
                    );
                    if let Pawn = piece.piece_type {
                        let from_row_index = m.from.row as i8;
                        let to_row_index = m.to.row as i8;
                        if to_row_index - from_row_index == 2 {
                            let in_between = Coordinate {
                                row: RowIndex::from((from_row_index + 1) as usize),
                                column: m.to.column,
                            };
                            Some(in_between)
                        } else if to_row_index - from_row_index == -2 {
                            let in_between = Coordinate {
                                row: RowIndex::from((from_row_index - 1) as usize),
                                column: m.to.column,
                            };
                            Some(in_between)
                        } else {
                            None // Only a double PawnMove can precede an en-passant
                        }
                    } else {
                        None // Only a PawnMove can precede an en-passant
                    }
                }
                _ => {
                    None // CastleMoves and TakeMoves cannot precede en-passant
                }
            }
        } else {
            // TODO: use start state
            None
        }
    }
}

// TODO: remove these implementations?
impl Deref for MoveRecords {
    type Target = Vec<MoveRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MoveRecords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for MoveRecords {
    fn default() -> Self {
        Self::new()
    }
}
