use crate::{
    board::{
        coordinates::{ColumnIndex, Coordinate, Move, RowIndex},
        piece::{
            Colour::{self, *},
            Piece,
        },
        BoardState,
    },
    display::Display,
    parsing::parse_coordinate,
    Player,
};
use std::io::{self, Write};

struct CliDisplay {}

impl Display for CliDisplay {
    fn get_unique_id(&self) -> u32 {
        1
    }
    fn display_board(&self, board_state: &BoardState) {
        print!("{}[2J", 27 as char);
        pretty_print(&board_state);
        println!("\n{:?}'s move\n", board_state.get_next_player());
    }
    fn display_checkmate(&self, winner: Colour) {
        println!("Checkmate!\n{:?} Wins", winner);
    }
}

pub struct InteractiveCliPlayer {}

impl InteractiveCliPlayer {
    pub fn new() -> Self {
        InteractiveCliPlayer {}
    }
}

impl Player for InteractiveCliPlayer {
    fn get_move(&self, board_state: &mut BoardState) -> io::Result<Move> {
        fn get_coordinate(message: &str) -> io::Result<Coordinate> {
            let mut buffer = String::new();
            loop {
                print!("{}", message);
                io::stdout().flush()?;
                io::stdin().read_line(&mut buffer)?;
                let coordinates = parse_coordinate(buffer.trim_end());
                buffer.clear();
                match coordinates {
                    Ok(c) => return Ok(c),
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
        let player = board_state.get_next_player();
        let from = loop {
            let possible_from = get_coordinate("Move from: ")?;
            let possible_moves = board_state.get_legal_moves_from(possible_from, player);
            if possible_moves.len() != 0 {
                break possible_from;
            } else {
                println!(
                    "Error: No moves available from {:?}{:?}",
                    possible_from.column, possible_from.row
                );
            }
        };
        let m = loop {
            let to = get_coordinate("Move to: ")?;
            let possible_move = Move { from, to };
            match board_state.is_legal_move(&possible_move) {
                Ok(()) => {
                    break possible_move;
                }
                Err(error) => println!("Error: {}", error),
            }
        };
        Ok(m)
    }
    fn get_display(&self) -> Box<dyn Display> {
        Box::new(CliDisplay {})
    }
}

fn square_colour(rank: RowIndex, file: ColumnIndex) -> Colour {
    let r = rank as u8;
    let f = file as u8;
    if f % 2 ^ r % 2 == 0 {
        White
    } else {
        Black
    }
}

fn print_square(square: Option<Piece>, rank: RowIndex, file: ColumnIndex) -> String {
    match square {
        Some(piece) => piece.to_string(),
        None => match square_colour(rank, file) {
            White => '◻',
            Black => '◼',
        }
        .to_string(),
    }
}

fn pretty_print(board_state: &BoardState) -> String {
    print!("  ");
    for &column in ColumnIndex::get_columns() {
        print!(" {:?}", column);
    }
    println!("");
    for &rank in RowIndex::get_rows() {
        print!("{:?}", rank);
        for (file, square) in board_state.board[rank] {
            print!(" {}", print_square(square, rank, file));
        }
        println!("");
    }
    String::from("")
}
