use enum_map::enum_map;

pub mod piece;
use piece::*;

pub mod coordinates;
use coordinates::*;

pub mod grid;
use grid::*;

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
