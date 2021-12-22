use crate::{
    board::{
        coordinates::{
            ColumnIndex::{self, *},
            Coordinate, Move,
            RowIndex::{self, *},
        },
        grid::Square,
        piece::{
            Colour::{self, *},
            Piece,
            PieceType::*,
        },
        BoardState,
    },
    display::Display,
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
            fn parse_coordinates(input: &str) -> Result<Coordinate, String> {
                if input.len() != 2 {
                    return Err(String::from("Coordinates must have exactly two characters"));
                };
                let input_bytes = input.as_bytes();
                let parse_row = |column: ColumnIndex| -> Result<Coordinate, String> {
                    match input_bytes[1] {
                        b'1' => Ok(Coordinate { row: _1, column }),
                        b'2' => Ok(Coordinate { row: _2, column }),
                        b'3' => Ok(Coordinate { row: _3, column }),
                        b'4' => Ok(Coordinate { row: _4, column }),
                        b'5' => Ok(Coordinate { row: _5, column }),
                        b'6' => Ok(Coordinate { row: _6, column }),
                        b'7' => Ok(Coordinate { row: _7, column }),
                        b'8' => Ok(Coordinate { row: _8, column }),
                        _ => Err(String::from(format!(
                            "Invalid row {}",
                            input_bytes[1] as char
                        ))),
                    }
                };
                match input_bytes[0] {
                    b'a' | b'A' => parse_row(A),
                    b'b' | b'B' => parse_row(B),
                    b'c' | b'C' => parse_row(C),
                    b'd' | b'D' => parse_row(D),
                    b'e' | b'E' => parse_row(E),
                    b'f' | b'F' => parse_row(F),
                    b'g' | b'G' => parse_row(G),
                    b'h' | b'H' => parse_row(H),
                    _ => Err(String::from(format!(
                        "Invalid column {}",
                        input_bytes[0] as char
                    ))),
                }
            }
            let mut buffer = String::new();
            loop {
                print!("{}", message);
                io::stdout().flush()?;
                io::stdin().read_line(&mut buffer)?;
                let coordinates = parse_coordinates(buffer.trim_end());
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
