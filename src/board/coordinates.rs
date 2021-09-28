use enum_map::Enum;

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

impl ColumnIndex {
    pub const fn get_columns() -> &'static [ColumnIndex; 8] {
        const COLUMNS: [ColumnIndex; 8] = [A, B, C, D, E, F, G, H];
        &COLUMNS
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub row: RowIndex,
    pub column: ColumnIndex,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}
