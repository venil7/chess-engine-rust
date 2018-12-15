use darkruby_chess::field;
use darkruby_chess::piece::{Piece,Color};

#[test]
fn create_new_empty_field() {
    let field = field::Field::empty();
    assert_eq!(field.to_string(), "[ ]");
}

#[test]
fn create_new_non_empty_field() {
    let pawn = Piece::pawn(Color::White);
    let field = field::Field::with(pawn);
    assert_eq!(field.to_string(), "[♙]");
}
