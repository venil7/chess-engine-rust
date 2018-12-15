use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
  White,
  Black,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
  Pawn(Color),
  Knight(Color),
  Rook(Color),
  Bishop(Color),
  Queen(Color),
  King(Color),
}

impl fmt::Display for Piece {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let piece_str = match self {
      Piece::Pawn(Color::White) => '♙',
      Piece::Pawn(Color::Black) => '♟',
      Piece::Knight(Color::White) => '♘',
      Piece::Knight(Color::Black) => '♞',
      Piece::Rook(Color::White) => '♖',
      Piece::Rook(Color::Black) => '♜',
      Piece::Bishop(Color::White) => '♗',
      Piece::Bishop(Color::Black) => '♝',
      Piece::Queen(Color::White) => '♕',
      Piece::Queen(Color::Black) => '♛',
      Piece::King(Color::White) => '♔',
      Piece::King(Color::Black) => '♚',
    };
    write!(formatter, "{}", piece_str)
  }
}
