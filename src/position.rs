#[derive(Clone, Copy, Debug)]
pub struct Position {
  pub row: i8,
  pub col: i8,
}

pub type Move = (Position, Position);

impl Position {
  pub fn from_index(index: usize) -> Position {
    let col = (index % 8) as i8;
    let row = (index / 8) as i8;
    Position { row, col }
  }

  pub fn to_index(&self) -> usize {
    (self.row * 8 + self.col) as usize
  }

  pub fn up(&self) -> Option<Position> {
    if self.row <= 0 {
      return None;
    }
    Some(Position {
      row: self.row - 1,
      col: self.col,
    })
  }

  pub fn down(&self) -> Option<Position> {
    if self.row >= 7 {
      return None;
    }
    Some(Position {
      row: self.row + 1,
      col: self.col,
    })
  }

  pub fn left(&self) -> Option<Position> {
    if self.col <= 0 {
      return None;
    }
    Some(Position {
      row: self.row,
      col: self.col - 1,
    })
  }
  pub fn right(&self) -> Option<Position> {
    if self.col >= 7 {
      return None;
    }
    Some(Position {
      row: self.row,
      col: self.col + 1,
    })
  }

  pub fn up_right(&self) -> Option<Position> {
    let up = self.up()?;
    up.right()
  }

  pub fn up_left(&self) -> Option<Position> {
    let up = self.left()?;
    up.left()
  }

  pub fn down_right(&self) -> Option<Position> {
    let down = self.down()?;
    down.right()
  }

  pub fn down_left(&self) -> Option<Position> {
    let down = self.left()?;
    down.left()
  }
}
