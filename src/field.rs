use crate::piece::Piece;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Field {
    Occupied(Piece),
    Empty,
}

impl fmt::Display for Field {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let cell_str = match self {
            Field::Occupied(ref piece) => piece.to_string(),
            Field::Empty => " ".to_string(),
        };
        write!(formatter, "[{}]", cell_str)
    }
}
