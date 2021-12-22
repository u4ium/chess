use std::convert::TryInto;
use std::str::FromStr;

use enum_map::EnumMap;

use super::{BoardState, Piece};

impl FromStr for BoardState {
    type Err = String;
    fn from_str(fen_string: &str) -> Result<Self, Self::Err> {
        let fields: Vec<_> = fen_string.split(" ").collect();
        let num_fields = fields.len();
        let [
            pieces, // Piece placement (from White's perspective). Each rank is described, starting with rank 8 and ending with rank 1; within each rank, the contents of each square are described from file "a" through file "h". Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the standard English names (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K"). White pieces are designated using upper-case letters ("PNBRQK") while black pieces use lowercase ("pnbrqk"). Empty squares are noted using digits 1 through 8 (the number of empty squares), and "/" separates ranks.
            next_player, // Active color. "w" means White moves next, "b" means Black moves next.
            _castling, // Castling availability. If neither side can castle, this is "-". Otherwise, this has one or more letters: "K" (White can castle kingside), "Q" (White can castle queenside), "k" (Black can castle kingside), and/or "q" (Black can castle queenside). A move that temporarily prevents castling does not negate this notation.
            _en_passant, // En passant target square in algebraic notation. If there's no en passant target square, this is "-". If a pawn has just made a two-square move, this is the position "behind" the pawn. This is recorded regardless of whether there is a pawn in position to make an en passant capture.[6]
            _halfmove_clock, // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[7]
            _number_of_full_moves, // Fullmove number: The number of the full move. It starts at 1, and is incremented after Black's move.
        ]: [&str; 6] = fields.try_into().map_err(|_| format!("FEN PARSE ERROR: wrong number of fields in record ({}/6)", num_fields))?;

        let rows: Vec<_> = pieces.split('/').collect();
        let num_rows = rows.len();
        let rows: [&str; 8] = rows
            .try_into()
            .map_err(|_| format!("FEN PARSE ERROR: wrong number of rows ({}/8)", num_rows))?;

        let row_maps = rows
            .iter()
            .map(|row| {
                let mut pieces: [Option<Piece>; 8] = Default::default();
                let mut index = 0;
                for c in row.chars() {
                    match c {
                        num_empty @ '1'..='8' => {
                            index += num_empty as usize - '0' as usize;
                        }
                        _ => {
                            pieces[index] = Piece::from_char(c)?;
                            index += 1;
                        }
                    }
                }
                Ok(EnumMap::from_array(pieces))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let board = EnumMap::from_array(row_maps.try_into().unwrap());

        let moves = match next_player {
            "b" => Vec::new(), // TODO: remove this field and replace with next player / number_of full_moves
            "w" => Vec::new(), // TODO: remove this field and replace with next player / number_of full_moves
            _ => {
                return Err(format!(
                    "FEN PARSE ERROR: next player must be 'b' or 'w' (not {})",
                    next_player
                ))
            }
        };
        Ok(Self { board, moves })
    }
}

#[cfg(test)]
mod tests {
    use super::BoardState;
    use std::str::FromStr;

    #[test]
    fn empty_board() {
        let expect: BoardState = BoardState {
            board: Default::default(),
            moves: Vec::new(),
        };
        let actual = BoardState::from_str("8/8/8/8/8/8/8/8 - - - - -").unwrap();
        assert_eq!(expect, actual);
    }
}
