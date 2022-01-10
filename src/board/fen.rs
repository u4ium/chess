use std::convert::TryInto;
use std::str::FromStr;

use enum_map::EnumMap;

use crate::parsing::parse_coordinate;

use super::{
    coordinates::{ColumnIndex, Coordinate, RowIndex},
    grid::Board,
    moves::MoveRecords,
    piece::{Colour, Movable},
    BoardState, CastlingAvailability,
    Colour::*,
    ColumnIndex::*,
    Piece,
};

fn parse_castling_availability(fen_castling_field: &str) -> Result<CastlingAvailability, String> {
    // TODO: prevent "buffer overflow"
    let fen_castling_field_chars: Vec<char> = fen_castling_field.chars().collect();
    let is_shredder = match fen_castling_field_chars[..] {
        [] => {
            return Err(String::from(
                "FEN PARSE ERROR: castling availability must not be blank",
            ))
        }
        ['-'] => return Ok(CastlingAvailability(Default::default())),
        ['k' | 'K' | 'q' | 'Q', ..] => false,
        _ => true,
    };

    let mut map: EnumMap<Colour, EnumMap<ColumnIndex, bool>> = Default::default();
    for c in fen_castling_field_chars {
        let (colour, file) = match c {
            'k' if !is_shredder => (Black, A),
            'K' if !is_shredder => (White, A),
            'q' if !is_shredder => (Black, H),
            'Q' if !is_shredder => (White, H),
            'a'..='h' if is_shredder => (Black, ColumnIndex::parse(c)?),
            'A'..='H' if is_shredder => (White, ColumnIndex::parse(c)?),
            _ => {
                return Err(format!(
                    "FEN PARSE ERROR: invalid character in availability field {}",
                    c
                ))
            }
        };
        if map[colour][file] {
            return Err(format!(
                "FEN PARSE ERROR: repeated character in availability field {}",
                c
            ));
        }
        map[colour][file] = true; // TODO: check repeats
    }

    Ok(CastlingAvailability(map))
}

fn parse_board(
    fen_pieces_field: &str,
    castling_availability: &CastlingAvailability,
) -> Result<Board, String> {
    // TODO: prevent "buffer overflow"
    let rows: Vec<_> = fen_pieces_field.split('/').collect();
    let num_rows = rows.len();
    let rows: [&str; 8] = rows
        .try_into()
        .map_err(|_| format!("FEN PARSE ERROR: wrong number of rows ({}/8)", num_rows))?;

    let row_maps = rows
        .iter() // TODO: use into_iter
        .enumerate()
        .map(|(row_index, row)| {
            let mut pieces: [Option<Piece>; 8] = Default::default();
            let mut index = 0;
            for c in row.chars() {
                match c {
                    num_empty @ '1'..='8' => {
                        index += num_empty as usize - '0' as usize;
                    }
                    _ => {
                        let mut new_piece = Piece::from_char(c)?;

                        let column_index = ColumnIndex::from(index);
                        let row_index = RowIndex::from(row_index);
                        new_piece.guess_and_set_is_moved(
                            row_index,
                            column_index,
                            castling_availability,
                        );

                        pieces[index] = new_piece;
                        index += 1;
                    }
                }
            }
            if index == 7 {
                Ok(EnumMap::from_array(pieces))
            } else {
                Err(format!(
                    "FEN PARSE ERROR: wrong number of pieces ({}/8) in row {}, ",
                    index, row_index
                ))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(Board(EnumMap::from_array(row_maps.try_into().unwrap())))
}

fn parse_en_passant_availability(
    fen_en_passant_availability_field: &str,
) -> Result<Option<Coordinate>, String> {
    if fen_en_passant_availability_field.len() == 1 {
        if let Some('-') = fen_en_passant_availability_field.chars().next() {
            return Ok(None);
        }
    }
    parse_coordinate(fen_en_passant_availability_field).and_then(|coordinate| Ok(Some(coordinate)))
}

impl FromStr for BoardState {
    type Err = String;
    fn from_str(fen_string: &str) -> Result<Self, Self::Err> {
        let fields: Vec<_> = fen_string.split(" ").collect();
        let num_fields = fields.len();
        let [

            pieces, // Piece placement (from White's perspective):
                    // Each rank is described, starting with rank 8 and ending with rank 1; 
                    // within each rank, the contents of each square are described from file "a" through file "h". 
                    // Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the standard English names:
                    //  - pawn = "P" 
                    //  - knight = "N" 
                    //  - bishop = "B" 
                    //  - rook = "R" 
                    //  - queen = "Q"
                    //  - king = "K" 
                    // White pieces are designated using upper-case letters ("PNBRQK") while black pieces use lowercase ("pnbrqk"). 
                    // Empty squares are noted using digits 1 through 8 (the number of empty squares), 
                    // and "/" separates ranks.

            active_player,  // Active player's colour:
                            //  - "w" means White moves next
                            //  - "b" means Black moves next.

            castling_availability,  // Castling availability: 
                                    // If neither side can castle, this is "-". 
                                    // Otherwise, this has one or more letters: 
                                    //  - In Standard FEN:
                                    //    - "K" (White can castle kingside)
                                    //    - "Q" (White can castle queenside)
                                    //    - "k" (Black can castle kingside)
                                    //    - "q" (Black can castle queenside)
                                    //  - OR in Shredder-FEN:
                                    //    - "A"-"H" (White rook that started and remains in file X can be castled with)
                                    //    - "a"-"h" (Black rook that started and remains in file X can be castled with)
                                    //    - As many letters as there are rooks that can still be castled with
                                    // A move that temporarily prevents castling does not negate this notation.

            en_passant, // En passant target square in algebraic notation. 
                        //  - If there's no en passant target square, this is "-". 
                        //  - If a pawn has just made a two-square move, this is the position "behind" the pawn. 
                        //  - This is recorded regardless of whether there is a pawn in position to make an en passant capture.

            _halfmove_clock, // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.

            _full_moves_number, // Fullmove number: The number of the full move. It starts at 1, and is incremented after Black's move.

        ]: [&str; 6] = fields.try_into().map_err(|_| format!("FEN PARSE ERROR: wrong number of fields in record ({}/6)", num_fields))?;

        let current_player = match active_player {
            "B" | "b" => Black,
            "W" | "w" => White,
            _ => {
                return Err(format!(
                    "FEN PARSE ERROR: next player must be 'b' or 'w' (not {})",
                    active_player
                ))
            }
        };

        let castling_availability = parse_castling_availability(castling_availability)?;
        let en_passant_availability = parse_en_passant_availability(en_passant)?;

        let board = parse_board(pieces, &castling_availability)?;

        let moves = MoveRecords::new(en_passant_availability);

        Ok(Self {
            current_player,
            board,
            moves,
            en_passant_availability,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BoardState;
    use std::str::FromStr;

    #[test]
    fn empty_board() {
        let expect: BoardState = BoardState {
            current_player: Default::default(),
            board: Default::default(),
            moves: Default::default(),
            en_passant_availability: None,
        };
        let actual = BoardState::from_str("8/8/8/8/8/8/8/8 w - - - -").unwrap();
        assert_eq!(expect, actual);
    }
}
