use crate::piece::Piece;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Field {
    piece: Option<Piece>,
}

impl Field {
    pub fn empty() -> Field {
        Field { piece: None }
    }

    pub fn with(piece: Piece) -> Field {
        Field { piece: Some(piece) }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let cell_str = match self.piece {
            Some(ref piece) => piece.to_string(),
            None => " ".to_string(),
        };
        write!(formatter, "[{}]", cell_str)
    }
}
