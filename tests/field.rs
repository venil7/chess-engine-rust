use darkruby_chess::field;
use darkruby_chess::piece::{Color, Piece};

#[test]
fn create_new_empty_field() {
    let field = field::Field::Empty;
    assert_eq!(field.to_string(), "[ ]");
}

#[test]
fn create_new_non_empty_field() {
    let pawn = Piece::Pawn(Color::White);
    let field = field::Field::Occupied(pawn);
    assert_eq!(field.to_string(), "[♙]");
}
