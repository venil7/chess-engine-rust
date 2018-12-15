use darkruby_chess::piece::{Color, Piece};

#[test]
fn pawn_displays_correctly() {
    let pawn = Piece::pawn(Color::White);
    assert_eq!(pawn.to_string(), "♙");
}
