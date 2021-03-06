use crate::board::Board;
use crate::field::Field;
use crate::piece::{piece_color, Color};
use crate::position::{Navigate, Position};

pub fn possible_moves(board: &Board, position: &Position, color: Color) -> Vec<Position> {
  vec![
    (*position).up().up_left(),
    (*position).up().up_right(),
    (*position).right().up_right(),
    (*position).right().down_right(),
    (*position).down().down_right(),
    (*position).down().down_left(),
    (*position).left().down_left(),
    (*position).left().up_left(),
  ]
  .iter()
  .fold(vec![], |mut acc, opt| {
    if let Some(pos) = opt {
      let field = board[*pos];
      match field {
        Field::Occupied(piece) if piece_color(&piece) == color.opposite() => acc.push(*pos),
        Field::Empty => acc.push(*pos),
        _ => (),
      }
    }
    acc
  })
}
