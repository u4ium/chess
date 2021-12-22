use crate::{
    board::{
        coordinates::{ColumnIndex::*, Move, RowIndex::*},
        grid::board_iterator,
        piece::{
            Colour::{self, *},
            PieceType::*,
        },
        BoardState,
    },
    display::Display,
    Player,
};

struct NoDisplay {}

impl Display for NoDisplay {
    fn get_unique_id(&self) -> u32 {
        0
    }
    fn display_board(&self, _: &BoardState) {}
    fn display_checkmate(&self, _: Colour) {}
}

#[derive(Debug)]
pub struct AiPlayer {
    depth: u8,
}

impl AiPlayer {
    pub fn new(depth: u8) -> Self {
        AiPlayer { depth }
    }
}

impl Player for AiPlayer {
    fn get_move(&self, board_state: &mut BoardState) -> std::io::Result<Move> {
        Ok(get_best_move(board_state, self.depth))
    }
    fn get_display(&self) -> Box<dyn Display> {
        Box::new(NoDisplay {})
    }
}

/// Note: panics if already in checkmate
fn get_best_move(board_state: &mut BoardState, depth: u8) -> Move {
    fn rec_helper(
        state: &mut BoardState,
        depth: u8,
        best_white: f64,
        best_black: f64,
    ) -> (Option<Move>, f64) {
        if depth == 0 {
            return (None, heuristic(state));
        }
        let current_player = state.get_next_player();
        let (mut current_best, other_best, worst) = match current_player {
            White => (best_white, best_black, -1.0),
            Black => (best_black, best_white, 1.0),
        };
        let mut result = (None, worst);
        for m in state.get_legal_moves(current_player) {
            let taken_from = match state.board[m.from.row][m.from.column] {
                Some(piece) if piece.piece_type == Pawn => {
                    // En-Passant: taken piece may not be at the "to" coordinate
                    piece.check_move(&state.board, &m, current_player).unwrap() // SAFE
                }
                _ => m.to,
            };
            let taken = state.do_move(&m, taken_from);
            match current_player {
                White => {
                    let (_, value) = rec_helper(state, depth - 1, current_best, other_best);
                    if value >= result.1 {
                        result.0 = Some(m);
                        result.1 = value;
                    }
                    if value >= other_best {
                        state.undo_move(taken_from, taken);
                        return result;
                    }
                    if value > current_best {
                        current_best = value;
                    }
                }
                Black => {
                    let (_, value) = rec_helper(state, depth - 1, other_best, current_best);
                    if value <= result.1 {
                        result.0 = Some(m);
                        result.1 = value;
                    }
                    if value <= other_best {
                        state.undo_move(taken_from, taken);
                        return result;
                    }
                    if value < current_best {
                        current_best = value;
                    }
                }
            }
            state.undo_move(taken_from, taken);
        }
        result
    }
    /// Return 1 for White Win, -1 for Black Win
    /// Otherwise, Return a number in range (-1, 1) estimating who is closer to winning
    fn heuristic(board_state: &BoardState) -> f64 {
        board_iterator().fold(0.0, |result, (&row, &column)| {
            let row_factor = match row {
                _1 | _8 => 0.85,
                _2 | _7 => 0.9,
                _3 | _6 => 0.95,
                _4 | _5 => 1.0,
            };
            let column_factor = match column {
                A | H => 0.85,
                B | G => 0.9,
                C | F => 0.95,
                D | E => 1.0,
            };
            match board_state.board[row][column] {
                // Some(piece) => result + piece.get_value(),
                Some(piece) => result + piece.get_value() * row_factor * column_factor,
                None => result,
            }
        })
    }
    let depth = if depth == 0 { 1 } else { depth };
    let (m, _) = rec_helper(board_state, depth * 2 - 1, -1.0, 1.0);
    m.unwrap_or_else(|| {
        panic!(
            "Cannot use AI to determine next move after Checkmate {:#?}",
            board_state.get_legal_moves(board_state.get_next_player())
        );
    })
}
