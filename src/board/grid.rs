use enum_map::EnumMap;
use itertools::{Itertools, Product};
use std::iter::repeat;
use std::slice;

use crate::board::coordinates::*;
use crate::board::piece::*;

pub type Square = Option<Piece>;
pub type Board = EnumMap<RowIndex, EnumMap<ColumnIndex, Square>>;

type BoardIterator = Product<slice::Iter<'static, RowIndex>, slice::Iter<'static, ColumnIndex>>;
pub fn board_iterator() -> BoardIterator {
    RowIndex::get_rows()
        .iter()
        .cartesian_product(ColumnIndex::get_columns().iter())
}

/// Return the coordinates between m.from and m.to
///
/// REQ: m is in a straight line
///
/// EXAMPLES:
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _1, column: A},
///     to: Coordinate {row: _4, column: D},
/// };
/// assert_eq!(squares_between(&m), vec![
///     Coordinate {row: _2, column: B},
///     Coordinate {row: _3, column: C}
/// ]);
/// ```
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _7, column: G},
///     to: Coordinate {row: _4, column: D},
/// };
/// assert_eq!(squares_between(&m), vec![
///     Coordinate {row: _6, column: F},
///     Coordinate {row: _5, column: E}
/// ]);
/// ```
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _7, column: G},
///     to: Coordinate {row: _4, column: G},
/// };
/// assert_eq!(squares_between(&m), vec![
///     Coordinate {row: _6, column: G},
///     Coordinate {row: _5, column: G}
/// ]);
/// ```
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _7, column: G},
///     to: Coordinate {row: _7, column: D},
/// };
/// assert_eq!(squares_between(&m), vec![
///     Coordinate {row: _7, column: F},
///     Coordinate {row: _7, column: E}
/// ]);
/// ```
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _7, column: D},
///     to: Coordinate {row: _7, column: G},
/// };
/// assert_eq!(squares_between(&m), vec![
///     Coordinate {row: _7, column: E},
///     Coordinate {row: _7, column: F}
/// ]);
/// ```
/// ```
/// use chess::{RowIndex::*, ColumnIndex::*, Coordinate, Move, squares_between};
/// let m = Move {
///     from: Coordinate {row: _1, column: A},
///     to: Coordinate {row: _2, column: B},
/// };
/// assert_eq!(squares_between(&m), vec![]);
/// ```
pub fn squares_between(m: &Move) -> Vec<Coordinate> {
    // Columns
    let to_column = m.to.column as usize;
    let from_column = m.from.column as usize;
    let columns = if from_column > to_column {
        &ColumnIndex::get_columns()[to_column + 1..from_column]
    } else if from_column < to_column {
        &ColumnIndex::get_columns()[from_column + 1..to_column]
    } else {
        &[]
    };
    // Rows
    let to_row = m.to.row as usize;
    let from_row = m.from.row as usize;
    let rows = if from_row > to_row {
        &RowIndex::get_rows()[to_row + 1..from_row]
    } else if from_row < to_row {
        &RowIndex::get_rows()[from_row + 1..to_row]
    } else {
        &[]
    };

    let to_coordinate = |(row, column): (&RowIndex, &ColumnIndex)| Coordinate {
        row: *row,
        column: *column,
    };
    // Equate sizes, reverse if needed, zip and map to Coordinates
    if (rows.len() == 0) ^ (columns.len() == 0) {
        if rows.len() == 0 {
            let e = &RowIndex::get_rows()[from_row];
            if from_column > to_column {
                repeat(e)
                    .zip(columns.iter().rev())
                    .map(to_coordinate)
                    .collect()
            } else {
                repeat(e).zip(columns.iter()).map(to_coordinate).collect()
            }
        } else {
            let e = &ColumnIndex::get_columns()[from_column];
            if from_row > to_row {
                rows.iter()
                    .rev()
                    .zip(repeat(e))
                    .map(to_coordinate)
                    .collect()
            } else {
                rows.iter().zip(repeat(e)).map(to_coordinate).collect()
            }
        }
    } else {
        match (from_column > to_column, from_row > to_row) {
            (true, true) => rows
                .iter()
                .rev()
                .zip(columns.iter().rev())
                .map(to_coordinate)
                .collect(),
            (true, false) => rows
                .iter()
                .zip(columns.iter().rev())
                .map(to_coordinate)
                .collect(),
            (false, true) => rows
                .iter()
                .rev()
                .zip(columns.iter())
                .map(to_coordinate)
                .collect(),
            (false, false) => rows.iter().zip(columns.iter()).map(to_coordinate).collect(),
        }
    }
}

/// REQ: m is in a straight line
pub fn has_no_pieces_between<'a>(
    board: &'a Board,
    m: &Move,
) -> Result<(), (&'a Piece, Coordinate)> {
    for coordinate in squares_between(m) {
        match &board[coordinate.row][coordinate.column] {
            None => {}
            Some(piece) => {
                return Err((piece, coordinate));
            }
        };
    }
    Ok(())
}
