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
  .map(|p| p.unwrap())
  .collect()
}

pub fn possible_moves_for_white(board: &Board, position: &Position) -> Vec<Option<Position>> {
  let first = position.up();
  let mut base_positions: Vec<Option<Position>> = vec![];
  if !occupied(board, first) {
    base_positions.push(first);
    if start_position(first, Color::White) {
      let second = first.up();
      if !occupied(board, second) {
        base_positions.push(second);
      }
    }
  }

  let mut attack: Vec<Option<Position>> = vec![position.up_left(), position.up_right()]
    .iter()
    .filter(|pos| attackable(board, **pos, Color::White))
    .map(|pos| *pos)
    .collect();

  base_positions.append(&mut attack);

  base_positions
}

pub fn possible_moves_for_black(board: &Board, position: &Position) -> Vec<Option<Position>> {
  let first = position.down();
  let mut base_positions: Vec<Option<Position>> = vec![];
  if !occupied(board, first) {
    base_positions.push(first);
    if start_position(first, Color::Black) {
      let second = first.down();
      if !occupied(board, second) {
        base_positions.push(second);
      }
    }
  }

  let mut attack: Vec<Option<Position>> = vec![position.down_left(), position.down_right()]
    .iter()
    .filter(|pos| attackable(board, **pos, Color::Black))
    .map(|pos| *pos)
    .collect();

  base_positions.append(&mut attack);

  base_positions
}

fn occupied(board: &Board, position: Option<Position>) -> bool {
  match position {
    None => false,
    Some(pos) => match board[pos] {
      Field::Empty => false,
      _ => true,
    },
  }
}

fn start_position(position: Option<Position>, color: Color) -> bool {
  match position {
    None => false,
    Some(pos) => match (pos, color) {
      (Position { row: 1, .. }, Color::Black) => true,
      (Position { row: 6, .. }, Color::White) => true,
      _ => false,
    },
  }
}

fn attackable(board: &Board, position: Option<Position>, color: Color) -> bool {
  match position {
    None => false,
    Some(pos) => match board[pos] {
      Field::Empty => false,
      Field::Occupied(piece) => piece_color(&piece) == color.opposite(),
    },
  }
}
