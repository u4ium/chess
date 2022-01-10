use enum_map::{enum_map, EnumMap};
use itertools::{Itertools, Product};
use std::ops::{Deref, DerefMut};
use std::slice;

use crate::board::coordinates::*;
use crate::board::piece::*;
use Colour::*;
use ColumnIndex::*;
use PieceType::*;
use RowIndex::*;

pub type Square = Option<Piece>;
pub type Row = EnumMap<ColumnIndex, Square>;
pub type BoardMap = EnumMap<RowIndex, Row>;

#[derive(Debug, PartialEq, Default)]
pub struct Board(pub BoardMap);

impl Board {
    pub fn new() -> Self {
        Board(enum_map! {
            _8 => enum_map!{
                A => Some(Piece::new(Rook, Black)),
                B => Some(Piece::new(Knight, Black)),
                C => Some(Piece::new(Bishop, Black)),
                D => Some(Piece::new(Queen, Black)),
                E => Some(Piece::new(King, Black)),
                F => Some(Piece::new(Bishop, Black)),
                G => Some(Piece::new(Knight, Black)),
                H => Some(Piece::new(Rook, Black)),
            },
            _7 => enum_map!{
                A => Some(Piece::new(Pawn, Black)),
                B => Some(Piece::new(Pawn, Black)),
                C => Some(Piece::new(Pawn, Black)),
                D => Some(Piece::new(Pawn, Black)),
                E => Some(Piece::new(Pawn, Black)),
                F => Some(Piece::new(Pawn, Black)),
                G => Some(Piece::new(Pawn, Black)),
                H => Some(Piece::new(Pawn, Black)),
            },
            _6 => enum_map!{
                A => None,
                B => None,
                C => None,
                D => None,
                E => None,
                F => None,
                G => None,
                H => None,
            },
            _5 => enum_map!{
                A => None,
                B => None,
                C => None,
                D => None,
                E => None,
                F => None,
                G => None,
                H => None,
            },
            _4 => enum_map!{
                A => None,
                B => None,
                C => None,
                D => None,
                E => None,
                F => None,
                G => None,
                H => None,
            },
            _3 => enum_map!{
                A => None,
                B => None,
                C => None,
                D => None,
                E => None,
                F => None,
                G => None,
                H => None,
            },
            _2 => enum_map!{
                A => Some(Piece::new(Pawn, White)),
                B => Some(Piece::new(Pawn, White)),
                C => Some(Piece::new(Pawn, White)),
                D => Some(Piece::new(Pawn, White)),
                E => Some(Piece::new(Pawn, White)),
                F => Some(Piece::new(Pawn, White)),
                G => Some(Piece::new(Pawn, White)),
                H => Some(Piece::new(Pawn, White)),
            },
            _1 => enum_map!{
                A => Some(Piece::new(Rook, White)),
                B => Some(Piece::new(Knight, White)),
                C => Some(Piece::new(Bishop, White)),
                D => Some(Piece::new(Queen, White)),
                E => Some(Piece::new(King, White)),
                F => Some(Piece::new(Bishop, White)),
                G => Some(Piece::new(Knight, White)),
                H => Some(Piece::new(Rook, White)),
            },
        })
    }
}

impl Deref for Board {
    type Target = BoardMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type BoardIterator = Product<slice::Iter<'static, RowIndex>, slice::Iter<'static, ColumnIndex>>;
pub fn board_iterator() -> BoardIterator {
    RowIndex::get_rows()
        .iter()
        .cartesian_product(ColumnIndex::get_columns().iter())
}

/// REQ: m is in a straight line
pub fn has_no_pieces_between<'a>(
    board: &'a Board,
    m: &Move,
) -> Result<(), (&'a Piece, Coordinate)> {
    for coordinate in m.squares_between() {
        match &board[coordinate.row][coordinate.column] {
            None => {}
            Some(piece) => {
                return Err((piece, coordinate));
            }
        };
    }
    Ok(())
}
