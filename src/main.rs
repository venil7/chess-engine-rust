use darkruby_chess::*;

fn main() {
  let board = board::Board::default();
  print!("{}", board.to_string());
}
