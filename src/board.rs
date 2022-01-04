use std::collections::{HashMap, HashSet};

use enum_map::enum_map;

pub mod piece;
use piece::*;

pub mod coordinates;
use coordinates::*;

pub mod grid;
use grid::*;

pub mod moves;
use moves::{
    CastlingAvailability,
    MoveRecord::{self, *},
    MoveRecords,
};

mod fen;

#[derive(Debug, PartialEq)]
pub struct BoardState {
    pub board: Board,
    pub moves: MoveRecords,
    pub castling_availability: CastlingAvailability,
    pub en_passant_availability: Option<Coordinate>,
}

use Colour::*;
use ColumnIndex::*;
use PieceType::*;
use RowIndex::*;

impl BoardState {
    // pub fn from_array(array: [[char; 8]; 8], player: Colour) -> Result<BoardState, String> {
    //     Ok(BoardState {
    //         board: EnumMap::from_array(
    //             array
    //                 .iter()
    //                 .map(|row| {
    //                     Ok(EnumMap::from_array(
    //                         row.iter()
    //                             .map(|&c| Piece::from_char(c))
    //                             .collect::<Result<Vec<_>, String>>()?
    //                             .try_into()
    //                             .unwrap(),
    //                     ))
    //                 })
    //                 .collect::<Result<Vec<_>, String>>()?
    //                 .try_into()
    //                 .unwrap(),
    //         ),
    //         moves: if player == White {
    //             vec![]
    //         } else {
    //             vec![Move {
    //                 // Bogus last move to set correct next_player
    //                 from: Coordinate { row: _1, column: A },
    //                 to: Coordinate { row: _1, column: A },
    //             }]
    //         },
    //     })
    // }

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
            moves: MoveRecords::new(),
            castling_availability: CastlingAvailability(enum_map! {
                White => enum_map!{
                    A => true,
                    B => false,
                    C => false,
                    D => false,
                    E => false,
                    F => false,
                    G => false,
                    H => true,
                },
                Black => enum_map!{
                    A => true,
                    B => false,
                    C => false,
                    D => false,
                    E => false,
                    F => false,
                    G => false,
                    H => true,
                },
            }),
            en_passant_availability: None,
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

    pub fn try_move(&mut self, m: Move) -> Result<(), String> {
        let current_player = self.get_next_player();
        let piece = self.board[m.from.row][m.from.column];
        let record = match piece {
            Some(p) => {
                p.check_move(&self.board, m, current_player, self.en_passant_availability)?
            }
            None => {
                return Err(String::from("No piece found to move"));
            }
        };
        if self.would_be_check(&record, current_player) {
            Err(String::from("Cannot move here: Check"))
        } else {
            Ok(self.do_move(record))
        }
    }

    pub fn do_move(&mut self, record: MoveRecord) {
        match record {
            SimpleMove { m, .. } => {
                self.board[m.to.row][m.to.column] =
                    self.board[m.from.row][m.from.column].take().moved(true);
            }
            TakeMove { m, taken_from, .. } => {
                self.board[taken_from.row][taken_from.column] = None;
                self.board[m.to.row][m.to.column] =
                    self.board[m.from.row][m.from.column].take().moved(true);
            }
            CastleMove(m1, m2) => {
                self.board[m1.to.row][m1.to.column] =
                    self.board[m1.from.row][m1.from.column].take().moved(true);
                self.board[m2.to.row][m2.to.column] =
                    self.board[m2.from.row][m2.from.column].take().moved(true);
            }
            PawnPromotion { m, to, .. } => {
                self.board[m.from.row][m.from.column] = None;
                self.board[m.to.row][m.to.column] = Some(Piece::new(to, self.get_next_player()));
            }
        }
        self.moves.push(record);
        self.recompute_en_passant_availability();
    }

    /// Note: Panics if self.moves is empty
    pub fn undo_move(&mut self) {
        let record = self
            .moves
            .pop()
            .expect("ERROR: Cannot undo moves, since none have been made");
        match record {
            SimpleMove { m, first_move } => {
                self.board[m.from.row][m.from.column] =
                    self.board[m.to.row][m.to.column].take().moved(!first_move);
            }
            TakeMove {
                m,
                taken,
                taken_from,
                first_move,
            } => {
                self.board[m.from.row][m.from.column] =
                    self.board[m.to.row][m.to.column].take().moved(!first_move);
                self.board[taken_from.row][taken_from.column] = Some(taken);
            }
            CastleMove(m1, m2) => {
                self.board[m1.from.row][m1.from.column] =
                    self.board[m1.to.row][m1.to.column].take().moved(false);
                self.board[m2.from.row][m2.from.column] =
                    self.board[m2.to.row][m2.to.column].take().moved(false);
            }
            PawnPromotion { m, taken, .. } => {
                self.board[m.to.row][m.to.column] = taken;
                self.board[m.from.row][m.from.column] = Some(Piece {
                    piece_type: Pawn,
                    colour: self.get_other_player(),
                    has_moved: true,
                });
            }
        }
        self.recompute_en_passant_availability();
    }

    pub fn is_checkmate(&mut self) -> bool {
        let player = self.get_next_player();
        self.is_in_check(player) && self.get_legal_moves(player).len() == 0
    }

    /// Note: will panic if King is not found
    pub fn is_in_check(&self, player: Colour) -> bool {
        let (&row, &column) = self
            .find_king(player)
            .expect(&format!("{:?} King not found {:#?}", player, self.board)[..]);
        let king_coordinates = Coordinate { row, column };
        let checking_moves = self.get_moves_to(king_coordinates, other_player(player));
        checking_moves.len() > 0
    }

    pub fn get_legal_moves_from(&mut self, from: Coordinate, by: Colour) -> Vec<Move> {
        match self.board[from.row][from.column] {
            Some(piece) if piece.colour == by => board_iterator()
                .filter_map(|(&row, &column)| {
                    let to = Coordinate { row, column };
                    let m = Move { from, to };
                    match piece.check_move(&self.board, m, by, self.en_passant_availability) {
                        Ok(record) if !self.would_be_check(&record, by) => Some(m),
                        _ => None,
                    }
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn is_legal_move(&mut self, m: Move) -> Result<(), String> {
        let player = self.get_next_player();
        match self.board[m.from.row][m.from.column] {
            None => Err(String::from("No piece to move")),
            Some(piece) => {
                let record =
                    piece.check_move(&self.board, m, player, self.en_passant_availability)?;
                if !self.would_be_check(&record, player) {
                    Ok(())
                } else {
                    Err(String::from("Cannot move here: Check"))
                }
            }
        }
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

    pub fn get_legal_moves_map(&mut self, by: Colour) -> HashMap<Coordinate, HashSet<Coordinate>> {
        self.get_legal_moves(by)
            .into_iter()
            .fold(Default::default(), |mut a, m| {
                a.entry(m.from).or_default().insert(m.to);
                a
            })
    }

    /// Note: Some of these moves may result in Check
    fn get_moves_to(&self, to: Coordinate, by: Colour) -> Vec<Coordinate> {
        board_iterator()
            .filter_map(|(&row, &column)| match self.board[row][column] {
                Some(piece) if piece.colour == by => {
                    let from = Coordinate { row, column };
                    let m = Move { from, to };
                    if piece
                        .check_move(&self.board, m, by, self.en_passant_availability)
                        .is_ok()
                    {
                        Some(from)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    fn recompute_en_passant_availability(&mut self) {
        self.en_passant_availability = self.moves.get_en_passant_availability(&self.board);
    }

    fn find_king(&self, player: Colour) -> Option<(&RowIndex, &ColumnIndex)> {
        board_iterator().find(|(&r, &c)| match self.board[r][c] {
            Some(Piece {
                piece_type: King,
                colour,
                ..
            }) if colour == player => true,
            _ => false,
        })
    }

    fn would_be_check(&mut self, record: &MoveRecord, by: Colour) -> bool {
        self.do_move(*record);
        let result = self.is_in_check(by);
        self.undo_move();
        result
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}
