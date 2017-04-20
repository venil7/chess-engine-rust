pub mod field;
pub mod board;
pub mod pieces;

fn main() {
  let board = board::Board::new();
  print!("{}", board.to_string());
}