use pieces::{Piece, Empty};

pub struct Field {
    piece: Box<Piece>,
}

impl Field {
    pub fn empty() -> Field {
        Field::with(Empty {})
    }

    pub fn with<T>(piece: T) -> Field
        where T: Piece + 'static
    {
        Field { piece: Box::new(piece) }
    }
}