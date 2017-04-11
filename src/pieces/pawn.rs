use std::fmt;
use pieces::{Piece, PieceType};

pub struct Pawn;
impl fmt::Display for Pawn {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", "♙")
    }
}

impl Piece for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
}