use crate::board::Board;
use crate::field::Field;
use crate::piece::{piece_color, Color};
use crate::position::{Navigate, Position};

pub fn possible_moves(board: &Board, position: &Position, color: Color) -> Vec<Position> {
  match color {
    Color::White => possible_moves_for_white(board, position),
    Color::Black => possible_moves_for_black(board, position),
  }
  .iter()
  .fold(vec![], |mut acc, opt| {
    if let Some(pos) = opt {
      match board[*pos] {
        Field::Occupied(piece) if piece_color(&piece) == color.opposite() => acc.push(*pos),
        Field::Empty => acc.push(*pos),
        _ => (),
      }
    }
    acc
  })
}

pub fn possible_moves_for_white(board: &Board, position: &Position) -> Vec<Option<Position>> {
  let first = (*position).up();
  let mut base_positions: Vec<Option<Position>> =
    vec![first, (*position).up_left(), (*position).up_right()];
  if let Some(pos) = first {
    if let Field::Empty = board[pos] {
      base_positions.push((*position).up().up());
    }
  }

  base_positions
}

pub fn possible_moves_for_black(board: &Board, position: &Position) -> Vec<Option<Position>> {
  let first = (*position).down();
  let mut base_positions: Vec<Option<Position>> =
    vec![first, (*position).down_left(), (*position).down_right()];
  if let Some(pos) = first {
    if let Field::Empty = board[pos] {
      base_positions.push((*position).down().down());
    }
  }

  base_positions
}
