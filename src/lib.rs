use std::io;

pub mod ai;
pub mod board;
pub mod cli;
pub mod display;

use board::{coordinates::Move, piece::Colour::*, BoardState};
use display::{Display, Displays};

pub trait Player {
    fn get_move(&self, board_state: &mut BoardState) -> io::Result<Move>;
    fn get_display(&self) -> Box<dyn Display>;
}

pub fn play_chess(white_player: &dyn Player, black_player: &dyn Player) -> io::Result<()> {
    let mut board_state = BoardState::new();
    let displays = Displays::new(vec![white_player.get_display(), black_player.get_display()]);
    while !board_state.is_checkmate() {
        displays.display_board(&board_state);
        let next_move = match board_state.get_next_player() {
            White => white_player.get_move(&mut board_state)?,
            Black => black_player.get_move(&mut board_state)?,
        };
        board_state
            .try_move(next_move)
            .unwrap_or_else(|error| panic!("ERROR: Impossible move: {}", error));
    }
    let winner = board_state.get_other_player();
    displays.display_checkmate(winner);
    Ok(())
}
