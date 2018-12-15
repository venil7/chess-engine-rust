use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
  White,
  Black,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
  Pawn(Color),
  Knight(Color),
  Rook(Color),
  Bishop(Color),
  Queen(Color),
  King(Color),
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
  type_: PieceType,
}

impl Piece {
  pub fn get_type(&self) -> PieceType {
    self.type_
  }
  pub fn pawn(color: Color) -> Piece {
    Piece {
      type_: PieceType::Pawn(color),
    }
  }
  pub fn knight(color: Color) -> Piece {
    Piece {
      type_: PieceType::Knight(color),
    }
  }
  pub fn rook(color: Color) -> Piece {
    Piece {
      type_: PieceType::Rook(color),
    }
  }
  pub fn bishop(color: Color) -> Piece {
    Piece {
      type_: PieceType::Bishop(color),
    }
  }
  pub fn queen(color: Color) -> Piece {
    Piece {
      type_: PieceType::Queen(color),
    }
  }
  pub fn king(color: Color) -> Piece {
    Piece {
      type_: PieceType::King(color),
    }
  }
}

impl fmt::Display for Piece {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let piece_str = match self.type_ {
      PieceType::Pawn(Color::White) => '♙',
      PieceType::Pawn(Color::Black) => '♟',
      PieceType::Knight(Color::White) => '♘',
      PieceType::Knight(Color::Black) => '♞',
      PieceType::Rook(Color::White) => '♖',
      PieceType::Rook(Color::Black) => '♜',
      PieceType::Bishop(Color::White) => '♗',
      PieceType::Bishop(Color::Black) => '♝',
      PieceType::Queen(Color::White) => '♕',
      PieceType::Queen(Color::Black) => '♛',
      PieceType::King(Color::White) => '♔',
      PieceType::King(Color::Black) => '♚',
    };
    write!(formatter, "{}", piece_str)
  }
}
