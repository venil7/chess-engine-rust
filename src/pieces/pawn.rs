use std::fmt;
use pieces::Piece;

pub struct Pawn;
impl Piece for Pawn {

}

impl fmt::Display for Pawn {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", "♙")
  }
}