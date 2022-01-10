use std::fmt::{self, Display, Formatter};
use std::iter::repeat;

use enum_map::Enum;

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RowIndex {
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    _1,
}
use RowIndex::*;

impl RowIndex {
    pub const fn get_rows() -> &'static [RowIndex; 8] {
        const ROWS: [RowIndex; 8] = [_8, _7, _6, _5, _4, _3, _2, _1];
        &ROWS
    }
}

impl From<usize> for RowIndex {
    fn from(index: usize) -> Self {
        Self::get_rows()[index]
    }
}

impl Display for RowIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ((8 - *self as u8) + b'0') as char)
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ColumnIndex {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
use ColumnIndex::*;

impl From<usize> for ColumnIndex {
    fn from(index: usize) -> Self {
        Self::get_columns()[index]
    }
}

impl Display for ColumnIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self as u8 + b'A') as char)
    }
}

impl ColumnIndex {
    pub const fn get_columns() -> &'static [ColumnIndex; 8] {
        const COLUMNS: [ColumnIndex; 8] = [A, B, C, D, E, F, G, H];
        &COLUMNS
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Coordinate {
    pub row: RowIndex,
    pub column: ColumnIndex,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
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
    pub fn squares_between(&self) -> Vec<Coordinate> {
        // Columns
        let to_column = self.to.column as usize;
        let from_column = self.from.column as usize;
        let columns = if from_column > to_column {
            &ColumnIndex::get_columns()[to_column + 1..from_column]
        } else if from_column < to_column {
            &ColumnIndex::get_columns()[from_column + 1..to_column]
        } else {
            &[]
        };
        // Rows
        let to_row = self.to.row as usize;
        let from_row = self.from.row as usize;
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
}
