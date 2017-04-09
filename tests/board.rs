extern crate chess;
use chess::board;

#[test]
fn create_new_board() {
  let board = board::Board::new();
    assert!(board.fields().len() == board::LENGTH);
}