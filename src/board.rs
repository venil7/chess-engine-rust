use crate::field::Field;
use crate::piece::{Color, Piece};
use std::fmt;

pub const LENGTH: usize = 64;

pub struct Board {
    pub fields: [Field; LENGTH],
}

impl Board {
    pub fn new() -> Board {
        let fields = [Field::empty(); LENGTH];
        Board { fields }
    }
    pub fn default() -> Board {
        let fields = [
            Field::with(Piece::rook(Color::Black)),
            Field::with(Piece::knight(Color::Black)),
            Field::with(Piece::bishop(Color::Black)),
            Field::with(Piece::queen(Color::Black)),
            Field::with(Piece::king(Color::Black)),
            Field::with(Piece::bishop(Color::Black)),
            Field::with(Piece::knight(Color::Black)),
            Field::with(Piece::rook(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::with(Piece::pawn(Color::Black)),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::empty(),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::pawn(Color::White)),
            Field::with(Piece::rook(Color::White)),
            Field::with(Piece::knight(Color::White)),
            Field::with(Piece::bishop(Color::White)),
            Field::with(Piece::queen(Color::White)),
            Field::with(Piece::king(Color::White)),
            Field::with(Piece::bishop(Color::White)),
            Field::with(Piece::knight(Color::White)),
            Field::with(Piece::rook(Color::White)),
        ];
        Board { fields }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str = self
            .fields
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (idx, field)| {
                let mut result = acc + &field.to_string();
                if (idx + 1) % 8 == 0 {
                    result = result + &"\n"
                }
                result
            });
        write!(formatter, "{}", str)
    }
}
