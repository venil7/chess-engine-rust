extern crate chess;
use chess::field;
use chess::pieces::pawn::Pawn;

#[test]
fn create_new_empty_field() {
    let field = field::Field::empty();
    assert_eq!(field.to_string(), "[ ]");
}

#[test]
fn create_new_non_empty_field() {
    let pawn = Pawn {};
    let field = field::Field::with(pawn);
    assert_eq!(field.to_string(), "[♙]");
}