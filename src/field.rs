use std::fmt;
use pieces::Piece;

type BoxedPiece = Box<Piece>;

pub struct Field {
    piece: Option<BoxedPiece>,
}

impl Field {
    pub fn empty() -> Field {
        Field { piece: None }
    }

    pub fn with<T>(piece: T) -> Field
        where T: Piece + 'static
    {
        let boxed_piece = Box::new(piece);
        Field { piece: Some(boxed_piece) }
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