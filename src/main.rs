use std::io::{self, Write};

mod ai;
mod lib;
use chess::{
    BoardState,
    Colour::{self, *},
    ColumnIndex::{self, *},
    Coordinate, Move,
    PieceType::*,
    RowIndex::{self, *},
    Square,
};

fn pretty_print(board_state: &BoardState) -> String {
    let print_square = |square: Square| -> String {
        let print_piece = |p: String, colour: Colour| -> String {
            match colour {
                White => p,
                Black => p.to_uppercase(),
            }
        };
        match square {
            Some(p) => print_piece(
                match p.piece_type {
                    Pawn => String::from("p"),
                    Rook => String::from("r"),
                    Knight => String::from("n"),
                    Bishop => String::from("b"),
                    King => String::from("k"),
                    Queen => String::from("q"),
                },
                p.colour,
            ),
            None => String::from("_"),
        }
    };
    print!("  ");
    for &column in ColumnIndex::get_columns() {
        print!(" {:?}", column);
    }
    println!("");
    for &row in RowIndex::get_rows() {
        print!("{:?}", row);
        for (_, square) in board_state.board[row] {
            print!(" {}", print_square(square));
        }
        println!("");
    }
    String::from("")
}

fn get_move_interactive(board_state: &mut BoardState) -> io::Result<Move> {
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

fn main() -> io::Result<()> {
    let mut board_state = BoardState::new();
    while !board_state.is_checkmate() {
        let player = board_state.get_next_player();
        print!("{}[2J", 27 as char);
        pretty_print(&board_state);
        println!();
        println!("{:?}'s move", player);
        println!();
        let m = match player {
            White => get_move_interactive(&mut board_state)?,
            //Black => get_move_interactive(&mut board_state)?,
            //White => ai::get_best_move(&mut board_state, 1),
            Black => ai::get_best_move(&mut board_state, 3),
        };
        board_state
            .try_move(m)
            .unwrap_or_else(|error| panic!("ERROR: Impossible move: {}", error));
    }
    println!("Checkmate!");
    println!("{:?} Wins", board_state.get_other_player());
    println!("Moves: {:#?}", board_state.moves); //Pretty-print Moves
    Ok(())
}
