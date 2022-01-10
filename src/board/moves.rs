use std::ops::Deref;

use super::{
    coordinates::{ColumnIndex, Coordinate, Move, RowIndex},
    grid::Board,
    piece::{
        Colour, Piece,
        PieceType::{self, Pawn},
    },
};
use enum_map::EnumMap;

type CastlingAvailabilityData = EnumMap<Colour, EnumMap<ColumnIndex, bool>>;

#[derive(Debug, PartialEq)]
pub struct CastlingAvailability(pub CastlingAvailabilityData);

impl Deref for CastlingAvailability {
    type Target = CastlingAvailabilityData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    CastleMove {
        rook_move: Move,
        king_move: Move,
    },
    PawnPromotion {
        /// the move that mas made
        m: Move,
        /// the type of piece the Pawn was promoted to
        to: PieceType,
        /// optionally, the piece that was taken by this Pawn
        taken: Option<Piece>,
    },
}

pub enum CastleDirection {
    UpFile,
    DownFile,
}

// TODO: use for public API: check_move
pub enum MoveSignal {
    Undo,
    Retire,
    ForceDraw,
    RequestDraw,
    Castle(CastleDirection),
    Make(Move),
    PromotePawnTo(PieceType),
}

#[derive(Debug, PartialEq)]
pub struct MoveRecords {
    start_en_passant_availability: Option<Coordinate>,
    moves: Vec<MoveRecord>,
}

impl MoveRecords {
    pub fn new(start_en_passant_availability: Option<Coordinate>) -> Self {
        Self {
            start_en_passant_availability,
            moves: Default::default(),
        }
    }

    pub fn get_en_passant_availability(&self, board: &Board) -> Option<Coordinate> {
        if let Some(move_record) = self.moves.last() {
            match move_record {
                MoveRecord::SimpleMove { m, .. } => {
                    let piece = board[m.to.row][m.to.column].expect(
                        "There should be a piece at the destination of the last SimpleMove",
                    );
                    if let Pawn = piece.piece_type {
                        let from_row_index = m.from.row as i8;
                        let to_row_index = m.to.row as i8;
                        let row_diff = to_row_index - from_row_index;
                        if row_diff.abs() != 2 {
                            return None; // Only a double PawnMove can precede an en-passant
                        }
                        let step = row_diff / 2; // A step in the direction of the double move
                        let square_in_between = Coordinate {
                            row: RowIndex::from((from_row_index + step) as usize),
                            column: m.to.column,
                        };
                        Some(square_in_between)
                    } else {
                        None // Only a PawnMove can precede an en-passant
                    }
                }
                _ => {
                    None // CastleMoves, TakeMoves and PawnPromotions cannot precede en-passant
                }
            }
        } else {
            self.start_en_passant_availability
        }
    }

    pub fn can_undo(&self) -> bool {
        self.moves.len() > 0
    }

    pub fn record_move(&mut self, record: MoveRecord) {
        self.moves.push(record);
    }

    pub fn pop_last_move(&mut self) -> Option<MoveRecord> {
        self.moves.pop()
    }
}

impl Default for MoveRecords {
    fn default() -> Self {
        Self::new(None)
    }
}
