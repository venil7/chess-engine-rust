use darkruby_chess::piece::{Color, Piece, PieceType};

#[test]
fn pawn_displays_correctly() {
    let pawn = Piece::pawn(Color::White);
    assert_eq!(pawn.to_string(), "♙");
}

#[test]
fn pawn_returns_correct_type() {
    let pawn = Piece::pawn(Color::White);
    assert_eq!(pawn.get_type(), PieceType::Pawn(Color::White));
}
