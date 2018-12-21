use crate::board::Board;
use crate::piece::Color;
use crate::position::Position;

pub fn possible_moves(board: &Board, position: &Position, color: Color) -> Vec<Position> {
  let opposite_color = color.opposite();
  [
    position.step_up_right_until(board, opposite_color),
    position.step_down_right_until(board, opposite_color),
    position.step_down_left_until(board, opposite_color),
    position.step_up_left_until(board, opposite_color),
  ]
  .iter()
  .flatten()
  .map(|p| *p)
  .collect()
}
