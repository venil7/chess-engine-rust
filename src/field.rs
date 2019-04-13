use crate::piece::{piece_color, Color, Piece};
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

pub fn field_color(field: &Field) -> Option<Color> {
  match field {
    Field::Occupied(piece) => Some(piece_color(piece)),
    Field::Empty => None,
  }
}

pub fn extract_piece(field: &Field) -> Piece {
  match field {
    Field::Occupied(piece) => *piece,
    _ => panic!("Can't extract piece from unnocupied field"),
  }
}

impl Field {
  pub fn color(&self) -> Option<Color> {
    field_color(self)
  }
  pub fn piece(&self) -> Piece {
    extract_piece(self)
  }
}
