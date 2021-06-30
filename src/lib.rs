use enum_map::{enum_map, Enum, EnumMap};
use itertools::{Itertools, Product};
use std::i8;
use std::iter::repeat;
use std::slice;

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

pub type Square = Option<Piece>;
pub type Board = EnumMap<RowIndex, EnumMap<ColumnIndex, Square>>;

type BoardIterator = Product<slice::Iter<'static, RowIndex>, slice::Iter<'static, ColumnIndex>>;
pub fn board_iterator() -> BoardIterator {
    RowIndex::get_rows()
        .iter()
        .cartesian_product(ColumnIndex::get_columns().iter())
}
#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Colour {
    Black,
    White,
}

fn other_player(player: Colour) -> Colour {
    match player {
        White => Black,
        Black => White,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
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
fn has_no_pieces_between<'a>(board: &'a Board, m: &Move) -> Result<(), (&'a Piece, Coordinate)> {
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

impl Piece {
    pub fn from_char(c: char) -> Result<Option<Piece>, String> {
        let piece_type = match c.to_ascii_lowercase() {
            'p' => Pawn,
            'r' => Rook,
            'n' => Knight,
            'b' => Bishop,
            'q' => Queen,
            'k' => King,
            '_' => {
                return Ok(None);
            }
            _ => return Err(format!("Invalid character for square {}", c)),
        };
        let colour = if c.is_uppercase() { Black } else { White };
        Ok(Some(Piece { piece_type, colour }))
    }

    pub fn new(piece_type: PieceType, colour: Colour) -> Piece {
        Piece { piece_type, colour }
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

pub struct BoardState {
    pub board: Board,
    pub moves: Vec<Move>,
}

use Colour::*;
use ColumnIndex::*;
use PieceType::*;
use RowIndex::*;

impl BoardState {
    pub fn from_array(array: [[char; 8]; 8], player: Colour) -> Result<BoardState, String> {
        Ok(BoardState {
            board: enum_map! {
                _8 => enum_map!{
                    A => Piece::from_char(array[_8 as usize][A as usize])?,
                    B => Piece::from_char(array[_8 as usize][B as usize])?,
                    C => Piece::from_char(array[_8 as usize][C as usize])?,
                    D => Piece::from_char(array[_8 as usize][D as usize])?,
                    E => Piece::from_char(array[_8 as usize][E as usize])?,
                    F => Piece::from_char(array[_8 as usize][F as usize])?,
                    G => Piece::from_char(array[_8 as usize][G as usize])?,
                    H => Piece::from_char(array[_8 as usize][H as usize])?,
                },
                _7 => enum_map!{
                    A => Piece::from_char(array[_7 as usize][A as usize])?,
                    B => Piece::from_char(array[_7 as usize][B as usize])?,
                    C => Piece::from_char(array[_7 as usize][C as usize])?,
                    D => Piece::from_char(array[_7 as usize][D as usize])?,
                    E => Piece::from_char(array[_7 as usize][E as usize])?,
                    F => Piece::from_char(array[_7 as usize][F as usize])?,
                    G => Piece::from_char(array[_7 as usize][G as usize])?,
                    H => Piece::from_char(array[_7 as usize][H as usize])?,
                },
                _6 => enum_map!{
                    A => Piece::from_char(array[_6 as usize][A as usize])?,
                    B => Piece::from_char(array[_6 as usize][B as usize])?,
                    C => Piece::from_char(array[_6 as usize][C as usize])?,
                    D => Piece::from_char(array[_6 as usize][D as usize])?,
                    E => Piece::from_char(array[_6 as usize][E as usize])?,
                    F => Piece::from_char(array[_6 as usize][F as usize])?,
                    G => Piece::from_char(array[_6 as usize][G as usize])?,
                    H => Piece::from_char(array[_6 as usize][H as usize])?,
                },
                _5 => enum_map!{
                    A => Piece::from_char(array[_5 as usize][A as usize])?,
                    B => Piece::from_char(array[_5 as usize][B as usize])?,
                    C => Piece::from_char(array[_5 as usize][C as usize])?,
                    D => Piece::from_char(array[_5 as usize][D as usize])?,
                    E => Piece::from_char(array[_5 as usize][E as usize])?,
                    F => Piece::from_char(array[_5 as usize][F as usize])?,
                    G => Piece::from_char(array[_5 as usize][G as usize])?,
                    H => Piece::from_char(array[_5 as usize][H as usize])?,
                },
                _4 => enum_map!{
                    A => Piece::from_char(array[_4 as usize][A as usize])?,
                    B => Piece::from_char(array[_4 as usize][B as usize])?,
                    C => Piece::from_char(array[_4 as usize][C as usize])?,
                    D => Piece::from_char(array[_4 as usize][D as usize])?,
                    E => Piece::from_char(array[_4 as usize][E as usize])?,
                    F => Piece::from_char(array[_4 as usize][F as usize])?,
                    G => Piece::from_char(array[_4 as usize][G as usize])?,
                    H => Piece::from_char(array[_4 as usize][H as usize])?,
                },
                _3 => enum_map!{
                    A => Piece::from_char(array[_3 as usize][A as usize])?,
                    B => Piece::from_char(array[_3 as usize][B as usize])?,
                    C => Piece::from_char(array[_3 as usize][C as usize])?,
                    D => Piece::from_char(array[_3 as usize][D as usize])?,
                    E => Piece::from_char(array[_3 as usize][E as usize])?,
                    F => Piece::from_char(array[_3 as usize][F as usize])?,
                    G => Piece::from_char(array[_3 as usize][G as usize])?,
                    H => Piece::from_char(array[_3 as usize][H as usize])?,
                },
                _2 => enum_map!{
                    A => Piece::from_char(array[_2 as usize][A as usize])?,
                    B => Piece::from_char(array[_2 as usize][B as usize])?,
                    C => Piece::from_char(array[_2 as usize][C as usize])?,
                    D => Piece::from_char(array[_2 as usize][D as usize])?,
                    E => Piece::from_char(array[_2 as usize][E as usize])?,
                    F => Piece::from_char(array[_2 as usize][F as usize])?,
                    G => Piece::from_char(array[_2 as usize][G as usize])?,
                    H => Piece::from_char(array[_2 as usize][H as usize])?,
                },
                _1 => enum_map!{
                    A => Piece::from_char(array[_1 as usize][A as usize])?,
                    B => Piece::from_char(array[_1 as usize][B as usize])?,
                    C => Piece::from_char(array[_1 as usize][C as usize])?,
                    D => Piece::from_char(array[_1 as usize][D as usize])?,
                    E => Piece::from_char(array[_1 as usize][E as usize])?,
                    F => Piece::from_char(array[_1 as usize][F as usize])?,
                    G => Piece::from_char(array[_1 as usize][G as usize])?,
                    H => Piece::from_char(array[_1 as usize][H as usize])?,
                },
            },
            moves: if player == White {
                vec![]
            } else {
                vec![Move {
                    // Bogus last move to set correct next_player
                    from: Coordinate { row: _1, column: A },
                    to: Coordinate { row: _1, column: A },
                }]
            },
        })
    }

    pub fn new() -> BoardState {
        BoardState {
            board: enum_map! {
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
            },
            moves: vec![],
        }
    }

    pub fn get_next_player(&self) -> Colour {
        match self.moves.len() % 2 {
            0 => White,
            _ => Black,
        }
    }

    pub fn get_other_player(&self) -> Colour {
        match self.moves.len() % 2 {
            0 => Black,
            _ => White,
        }
    }

    pub fn try_move(&mut self, m: Move) -> Result<Option<Piece>, String> {
        let current_player = self.get_next_player();

        let piece = self.board[m.from.row][m.from.column];
        let taken_from = match piece {
            Some(p) => p.check_move(&self.board, &m, current_player)?,
            None => {
                return Err(String::from("No piece found to move"));
            }
        };
        if self.would_be_check(&m, current_player, taken_from) {
            Err(String::from("Cannot move here: Check"))
        } else {
            Ok(self.do_move(&m, taken_from))
        }
    }

    pub fn do_move(&mut self, m: &Move, taken_from: Coordinate) -> Option<Piece> {
        let piece = self.board[m.from.row][m.from.column];
        let taken = self.board[taken_from.row][taken_from.column];
        self.board[taken_from.row][taken_from.column] = None;
        self.board[m.to.row][m.to.column] = piece;
        self.board[m.from.row][m.from.column] = None;
        self.moves.push(*m);
        taken
    }

    /// Note: Panics if self.moves is empty
    pub fn undo_move(&mut self, taken_from: Coordinate, taken: Option<Piece>) {
        let m = self.moves.pop().unwrap_or_else(|| {
            panic!("ERROR: Cannot undo moves, since none have been made");
        });
        let piece = self.board[m.to.row][m.to.column];
        self.board[m.to.row][m.to.column] = None;
        self.board[taken_from.row][taken_from.column] = taken;
        self.board[m.from.row][m.from.column] = piece;
    }

    pub fn is_checkmate(&mut self) -> bool {
        let player = self.get_next_player();
        self.is_in_check(player) && self.get_legal_moves(player).len() == 0
    }
    /// Note: will panic if King is not found
    pub fn is_in_check(&self, player: Colour) -> bool {
        let (&row, &column) = self.find_king(player).unwrap_or_else(|| {
            panic!("{:?} King not found {:#?}", player, self.board);
        });
        let king_coordinates = Coordinate { row, column };
        let checking_moves = self.get_moves_to(king_coordinates, other_player(player));
        checking_moves.len() > 0
    }

    fn find_king(&self, player: Colour) -> Option<(&RowIndex, &ColumnIndex)> {
        board_iterator().find(|(&r, &c)| match self.board[r][c] {
            Some(Piece {
                piece_type: King,
                colour,
            }) if colour == player => true,
            _ => false,
        })
    }
    pub fn get_legal_moves_from(&mut self, from: Coordinate, by: Colour) -> Vec<Move> {
        match self.board[from.row][from.column] {
            Some(piece) if piece.colour == by => board_iterator()
                .filter_map(|(&row, &column)| {
                    let to = Coordinate { row, column };
                    let m = Move { from, to };
                    match piece.check_move(&self.board, &m, by) {
                        Ok(taken_from) if !self.would_be_check(&m, by, taken_from) => Some(m),
                        _ => None,
                    }
                })
                .collect(),
            _ => vec![],
        }
    }

    /// Note: Some of these moves may result in Check
    fn get_moves_to(&self, to: Coordinate, by: Colour) -> Vec<Coordinate> {
        board_iterator()
            .filter_map(|(&row, &column)| match self.board[row][column] {
                Some(piece) if piece.colour == by => {
                    let from = Coordinate { row, column };
                    let m = Move { from, to };
                    if piece.check_move(&self.board, &m, by).is_ok() {
                        Some(from)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Note: Expensive operation
    ///
    /// Examples:
    /// ```
    /// use chess::{BoardState, ColumnIndex::*, RowIndex::*, Move, Coordinate, Colour::*};
    /// let mut board_state = BoardState::from_array([
    ///     ['R', 'N', 'q', '_', 'K', 'B', 'N', 'R'],
    ///     ['P', 'P', '_', '_', 'P', 'P', 'P', '_'],
    ///     ['_', '_', 'P', '_', '_', '_', '_', '_'],
    ///     ['_', 'Q', '_', 'P', '_', '_', '_', '_'],
    ///     ['_', '_', '_', '_', '_', '_', '_', '_'],
    ///     ['_', '_', '_', '_', 'p', 'p', 'p', '_'],
    ///     ['p', '_', 'p', 'p', '_', '_', '_', 'p'],
    ///     ['r', 'n', 'b', '_', 'k', '_', 'n', 'r'],
    /// ], Black).unwrap();
    /// assert_eq!(board_state.get_legal_moves(Black).len(), 0);
    /// ```
    pub fn get_legal_moves(&mut self, by: Colour) -> Vec<Move> {
        board_iterator()
            .map(|(&row, &column)| self.get_legal_moves_from(Coordinate { row, column }, by))
            .flatten()
            .collect()
    }
    fn would_be_check(&mut self, m: &Move, by: Colour, taken_from: Coordinate) -> bool {
        // TODO: add taken (En-Passant)
        let taken = self.do_move(m, taken_from);
        let result = self.is_in_check(by);
        self.undo_move(taken_from, taken);
        result
    }

    pub fn is_legal_move(&mut self, m: &Move) -> Result<(), String> {
        let player = self.get_next_player();
        match self.board[m.from.row][m.from.column] {
            None => Err(String::from("No piece to move")),
            Some(piece) => {
                let taken_from = piece.check_move(&self.board, m, player)?;
                if !self.would_be_check(&m, player, taken_from) {
                    Ok(())
                } else {
                    Err(String::from("Cannot move here: Check"))
                }
            }
        }
    }
}
