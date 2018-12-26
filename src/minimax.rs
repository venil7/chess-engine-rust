use crate::board::Board;
use crate::eval::score;
use crate::eval::Eval;
use crate::field::Field;
use crate::piece::{Color, Piece};
use crate::position::Move;
use rayon::prelude::*;
use std::cmp::Ordering;

const MAX_SCORE: f32 = 100.00;
const DEPTH: usize = 6;
fn xor<T>(a: Option<T>, b: Option<T>) -> Option<T> {
  match (a, b) {
    (None, b) => b,
    (a, None) => a,
    _ => None,
  }
}

pub fn winner(board: &Board) -> Option<Color> {
  board.fields.iter().fold(None, |acc, field| match field {
    Field::Occupied(Piece::King(color)) => xor(Some(*color), acc),
    _ => acc,
  })
}
pub fn minimax(board: &Board, depth: usize, color: Color, mv: Option<Move>) -> Eval {
  let winner = winner(&board);
  match (winner, mv, depth) {
    (Some(Color::White), Some(mv), _) => (mv, depth as f32 - MAX_SCORE),
    (Some(Color::Black), Some(mv), _) => (mv, MAX_SCORE - depth as f32),
    (_, Some(mv), 0) => (mv, score(&board, color)),
    (_, _, depth) => {
      let possible_moves = board.possible_moves(&color);
      let mut eval_moves: Vec<Eval> = possible_moves
        .par_iter()
        .map(|pmv| {
          let board_ = board.make_move(*pmv);
          let (_, score) = minimax(&board_, depth - 1, color.opposite(), Some(*pmv));
          (*pmv, score)
        })
        .collect();
      eval_moves.sort_by(sort_by_color(color));
      eval_moves[0]
    }
  }
}

pub fn cpu(board: &Board) -> Board {
  let (mv, _) = minimax(board, DEPTH, Color::Black, None);
  board.make_move(mv)
}

fn cmp(a: &f32, b: &f32) -> Ordering {
  if a < b {
    return Ordering::Less;
  } else if a > b {
    return Ordering::Greater;
  } else {
    return Ordering::Equal;
  }
}
fn sort_by_color(color: Color) -> (fn(&Eval, &Eval) -> Ordering) {
  match color {
    Color::Black => |(_, score1), (_, score2)| cmp(score1, score2),
    Color::White => |(_, score1), (_, score2)| cmp(score1, score2),
  }
}
