use darkruby_chess::minimax::cpu;
use darkruby_chess::*;

fn main() {
  let board = board::Board::default();
  let board = cpu(&board);
  print!("{}", board.to_string());
}
