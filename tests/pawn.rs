extern crate chess;
use chess::pieces::pawn::Pawn;
use chess::pieces::{Piece, PieceType};

#[test]
fn pawn_displays_correctly() {
    let pawn = Pawn {};
    assert_eq!(pawn.to_string(), "♙");
}

#[test]
fn pawn_returns_correct_type() {
    let pawn = Pawn {};
    assert_eq!(pawn.get_type(), PieceType::Pawn);
}