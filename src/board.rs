use std::collections::{HashMap, HashSet};

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

use ColumnIndex::*;
use RowIndex::*;

mod fen;

#[derive(Debug, PartialEq)]
pub struct BoardState {
    pub current_player: Colour,
    pub board: Board,
    pub moves: MoveRecords,
    pub en_passant_availability: Option<Coordinate>,
}

use Colour::*;
use PieceType::*;

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
            current_player: White,
            board: Board::new(),
            moves: MoveRecords::new(None),
            en_passant_availability: None,
        }
    }

    pub fn get_next_player(&self) -> Colour {
        self.current_player
    }

    pub fn get_other_player(&self) -> Colour {
        !self.current_player
    }

    pub fn get_move_result(&self, m: Move, by: Colour) -> Result<MoveRecord, String> {
        let piece = match self.board[m.from.row][m.from.column] {
            Some(p) => p,
            None => {
                return Err(String::from("Cannot move an empty square"));
            }
        };

        if piece.colour != by {
            return Err(String::from("Cannot move opponent's piece"));
        }

        if m.from == m.to {
            return Err(String::from("Must move piece"));
        }

        let destination_square = self.board[m.to.row][m.to.column];
        match destination_square {
            Some(t) if t.colour == by => {
                return Err(String::from("Cannot take own piece"));
            }
            _ => {}
        }

        let first_move = !piece.has_moved;
        let return_move_record = || match destination_square {
            Some(taken) => Ok(MoveRecord::TakeMove {
                m,
                taken,
                taken_from: m.to,
                first_move,
            }),
            None => Ok(MoveRecord::SimpleMove { m, first_move }),
        };
        let check_path_and_return_move_record = |m| match has_no_pieces_between(&self.board, &m) {
            Ok(()) => return_move_record(),
            Err((p, c)) => Err(format!(
                "Cannot move here: blocked at ({:?}, {:?}), by {:?}",
                c.row, c.column, p
            )),
        };

        let d_row = (m.to.row as i8) - (m.from.row as i8);
        let d_column = (m.to.column as i8) - (m.from.column as i8);

        let row_increment = if by == White { -1 } else { 1 };
        match piece.piece_type {
            Pawn => {
                let other_player = !by;
                let end_row = other_player.home_rank();
                match d_column {
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
                                if first_move {
                                    if self.board[in_between][m.from.column].is_none() {
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
                            None if self.en_passant_availability == Some(m.to) => {
                                let taken_from = Coordinate {
                                    column: m.to.column,
                                    row: RowIndex::from((m.to.row as i8 - row_increment) as usize),
                                };
                                let taken = self.board[taken_from.row][taken_from.column].expect("There should be a piece at the en-passant availability location.");
                                Ok(MoveRecord::TakeMove {
                                    m,
                                    taken,
                                    taken_from,
                                    first_move,
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
                }
            }
            Rook => match (d_row, d_column) {
                (_, 0) | (0, _) => check_path_and_return_move_record(m),
                _ => Err(String::from("Cannot move Rook here: not a straight line")),
            },
            Knight => match (d_row.abs(), d_column.abs()) {
                (1, 2) => return_move_record(),
                (2, 1) => return_move_record(),
                _ => Err(String::from("Cannot move Knight here: not in L pattern")),
            },
            Bishop => match (d_row.abs(), d_column.abs()) {
                (r, c) if r == c => check_path_and_return_move_record(m),
                _ => Err(String::from("Cannot move Bishop here: not a diagonal line")),
            },
            Queen => match (d_row.abs(), d_column.abs()) {
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
                    if piece.has_moved {
                        return Err(String::from("Cannot castle: King has been moved"));
                    }
                    // find target rook
                    let rook_coordinates = Coordinate {
                        row: m.from.row,
                        column: if direction == 2 { H } else { A },
                    };
                    let rook = match self.board[rook_coordinates.row][rook_coordinates.column] {
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
                    match has_no_pieces_between(&self.board, &m) {
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

    pub fn try_move(&mut self, m: Move) -> Result<(), String> {
        let current_player = self.get_next_player();
        let record = self.get_move_result(m, current_player)?;
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
            CastleMove {
                rook_move,
                king_move,
            } => {
                self.board[rook_move.to.row][rook_move.to.column] = self.board[rook_move.from.row]
                    [rook_move.from.column]
                    .take()
                    .moved(true);
                self.board[king_move.to.row][king_move.to.column] = self.board[king_move.from.row]
                    [king_move.from.column]
                    .take()
                    .moved(true);
            }
            PawnPromotion { m, to, .. } => {
                self.board[m.from.row][m.from.column] = None;
                self.board[m.to.row][m.to.column] = Some(Piece {
                    piece_type: to,
                    colour: self.current_player,
                    has_moved: true,
                });
            }
        }
        self.current_player = !self.current_player;
        self.moves.record_move(record);
        self.recompute_en_passant_availability();
    }

    /// Note: Panics if self.moves is empty
    pub fn undo_move(&mut self) {
        let record = self
            .moves
            .pop_last_move()
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
            CastleMove {
                rook_move,
                king_move,
            } => {
                self.board[rook_move.from.row][rook_move.from.column] = self.board
                    [rook_move.to.row][rook_move.to.column]
                    .take()
                    .moved(false);
                self.board[king_move.from.row][king_move.from.column] = self.board
                    [king_move.to.row][king_move.to.column]
                    .take()
                    .moved(false);
            }
            PawnPromotion { m, taken, .. } => {
                self.board[m.to.row][m.to.column] = taken;
                self.board[m.from.row][m.from.column] = Some(Piece {
                    piece_type: Pawn,
                    colour: !self.current_player,
                    has_moved: true,
                });
            }
        }
        self.current_player = !self.current_player;
        self.recompute_en_passant_availability();
    }

    pub fn is_checkmate(&mut self) -> bool {
        let player = self.get_next_player();
        self.is_in_check(player) && self.get_legal_moves(player).len() == 0
    }

    /// Note: will panic if King is not found
    pub fn is_in_check(&self, player: Colour) -> bool {
        let king_coordinates = self
            .find_king(player)
            .expect(&format!("{:?} King not found {:#?}", player, self.board)[..]);
        self.has_moves_to(king_coordinates, !player)
    }

    pub fn get_legal_moves_from(&mut self, from: Coordinate, by: Colour) -> Vec<Move> {
        match self.board[from.row][from.column] {
            Some(piece) if piece.colour == by => board_iterator()
                .filter_map(|(&row, &column)| {
                    let to = Coordinate { row, column };
                    let m = Move { from, to };
                    match self.get_move_result(m, by) {
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
        self.get_move_result(m, player).and_then(|record| {
            if !self.would_be_check(&record, player) {
                Ok(())
            } else {
                Err(String::from("Cannot move here: Check"))
            }
        })
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
    fn has_moves_to(&self, to: Coordinate, by: Colour) -> bool {
        for (&row, &column) in board_iterator() {
            let from = Coordinate { row, column };
            if self.get_move_result(Move { from, to }, by).is_ok() {
                return true;
            }
        }
        false
    }

    fn recompute_en_passant_availability(&mut self) {
        self.en_passant_availability = self.moves.get_en_passant_availability(&self.board);
    }

    fn find_king(&self, player: Colour) -> Option<Coordinate> {
        board_iterator()
            .find(|(&r, &c)| match self.board[r][c] {
                Some(Piece {
                    piece_type: King,
                    colour,
                    ..
                }) if colour == player => true,
                _ => false,
            })
            .map(|(&row, &column)| Coordinate { row, column })
    }

    fn would_be_check(&mut self, record: &MoveRecord, by: Colour) -> bool {
        if let CastleMove { king_move, .. } = record {
            // cannot castle out of check
            if self.is_in_check(by) {
                return true;
            }
            // cannot castle through check
            for square in king_move.squares_between() {
                let imaginary_king_move = SimpleMove {
                    m: Move {
                        from: king_move.from,
                        to: square,
                    },
                    first_move: true, // this is always true for a valid CastleMove
                };
                if self.would_be_check(&imaginary_king_move, by) {
                    return true;
                }
            }
        }
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
