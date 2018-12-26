use crate::board::Board;
use crate::field::Field;
use crate::piece::piece_color;
use crate::piece::{Color, Piece};
use crate::position::{Move, Position};

pub fn pawn_weight(position: Position, color: Color) -> f32 {
  let piece_weight: f32 = 1.0;
  let coefficient: f32 = 0.1;
  match color {
    Color::White => (piece_weight + (7.0 - (position.row as f32)) * coefficient),
    Color::Black => (piece_weight + (position.row as f32) * coefficient),
  }
}

pub fn weight(piece: Piece, position: Position) -> f32 {
  match piece {
    Piece::Pawn(color) => pawn_weight(position, color),
    Piece::Bishop(_) => 2.0,
    Piece::Rook(_) => 2.0,
    Piece::Knight(_) => 3.0,
    Piece::Queen(_) => 5.0,
    Piece::King(_) => 10.0,
  }
}

pub type Eval = (Move, f32);

pub fn score(board: &Board, color: Color) -> f32 {
  board
    .fields
    .iter()
    .enumerate()
    .fold(0.0, |acc, (index, field)| match field {
      Field::Occupied(piece) => {
        let w = weight(*piece, Position::from_index(index));
        if piece_color(piece) == color {
          acc + w
        } else {
          acc - w
        }
      }
      _ => acc,
    })
}
