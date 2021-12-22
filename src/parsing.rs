use crate::board::coordinates::{
    ColumnIndex::{self, *},
    Coordinate,
    RowIndex::*,
};

pub fn parse_coordinate(input: &str) -> Result<Coordinate, String> {
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
