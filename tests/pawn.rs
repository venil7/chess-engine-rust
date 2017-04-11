extern crate chess;
use chess::pieces::pawn::Pawn;

#[test]
fn pawn_displays_correctly() {
    let pawn = Pawn {};
    assert_eq!(pawn.to_string(), "♙");
}