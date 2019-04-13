use crate::board::Board;
use crate::field::Field;
use crate::piece::Color;

#[derive(Clone, Copy, Debug)]
pub struct Position {
  pub row: i8,
  pub col: i8,
}

pub type Move = (Position, Position);

pub trait Navigate {
  fn up(&self) -> Option<Position>;
  fn down(&self) -> Option<Position>;
  fn left(&self) -> Option<Position>;
  fn right(&self) -> Option<Position>;
  fn up_right(&self) -> Option<Position>;
  fn down_right(&self) -> Option<Position>;
  fn up_left(&self) -> Option<Position>;
  fn down_left(&self) -> Option<Position>;
}

impl Navigate for Option<Position> {
  fn up(&self) -> Option<Position> {
    let pos = (*self)?;
    if pos.row <= 0 {
      return None;
    }
    Some(Position {
      row: pos.row - 1,
      col: pos.col,
    })
  }

  fn down(&self) -> Option<Position> {
    let pos = (*self)?;
    if pos.row >= 7 {
      return None;
    }
    Some(Position {
      row: pos.row + 1,
      col: pos.col,
    })
  }

  fn left(&self) -> Option<Position> {
    let pos = (*self)?;
    if pos.col <= 0 {
      return None;
    }
    Some(Position {
      row: pos.row,
      col: pos.col - 1,
    })
  }
  fn right(&self) -> Option<Position> {
    let pos = (*self)?;
    if pos.col >= 7 {
      return None;
    }
    Some(Position {
      row: pos.row,
      col: pos.col + 1,
    })
  }

  fn up_right(&self) -> Option<Position> {
    self.up().right()
  }

  fn up_left(&self) -> Option<Position> {
    self.up().left()
  }

  fn down_right(&self) -> Option<Position> {
    self.down().right()
  }

  fn down_left(&self) -> Option<Position> {
    self.down().left()
  }
}

impl Navigate for Position {
  fn up(&self) -> Option<Position> {
    Some(*self).up()
  }
  fn down(&self) -> Option<Position> {
    Some(*self).down()
  }
  fn left(&self) -> Option<Position> {
    Some(*self).left()
  }
  fn right(&self) -> Option<Position> {
    Some(*self).right()
  }

  fn up_right(&self) -> Option<Position> {
    Some(*self).up_right()
  }

  fn up_left(&self) -> Option<Position> {
    Some(*self).up_left()
  }

  fn down_right(&self) -> Option<Position> {
    Some(*self).down_right()
  }

  fn down_left(&self) -> Option<Position> {
    Some(*self).down_left()
  }
}

impl Position {
  pub fn from_index(index: usize) -> Position {
    let col = (index % 8) as i8;
    let row = (index / 8) as i8;
    Position { row, col }
  }

  pub fn to_index(&self) -> usize {
    (self.row * 8 + self.col) as usize
  }

  pub fn step_until(
    &self,
    stepper: fn(Position) -> Option<Position>,
    board: &Board,
    color: Color,
  ) -> Vec<Position> {
    let mut current = self.clone();
    let mut res: Vec<Position> = vec![];
    while let Some(pos) = stepper(current) {
      match board[pos] {
        Field::Empty => res.push(pos),
        Field::Occupied(piece) if (piece.color() == color) => {
          res.push(pos);
          break;
        }
        _ => break,
      }
      current = pos;
    }
    res
  }

  pub fn step_up_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.up(), board, color)
  }
  pub fn step_down_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.down(), board, color)
  }
  pub fn step_left_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.left(), board, color)
  }
  pub fn step_right_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.right(), board, color)
  }
  pub fn step_up_right_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.up_right(), board, color)
  }
  pub fn step_down_right_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.down_right(), board, color)
  }
  pub fn step_up_left_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.up_left(), board, color)
  }
  pub fn step_down_left_until(&self, board: &Board, color: Color) -> Vec<Position> {
    self.step_until(|p| p.down_left(), board, color)
  }
}
