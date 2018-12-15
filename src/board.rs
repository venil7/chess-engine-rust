use crate::field::Field;
use crate::piece::{Color, Piece};
use std::fmt;

pub const LENGTH: usize = 64;

pub struct Board {
    pub fields: [Field; LENGTH],
}

impl Board {
    pub fn new() -> Board {
        let fields = [Field::Empty; LENGTH];
        Board { fields }
    }
    pub fn default() -> Board {
        let fields = [
            Field::Occupied(Piece::Rook(Color::Black)),
            Field::Occupied(Piece::Knight(Color::Black)),
            Field::Occupied(Piece::Bishop(Color::Black)),
            Field::Occupied(Piece::Queen(Color::Black)),
            Field::Occupied(Piece::King(Color::Black)),
            Field::Occupied(Piece::Bishop(Color::Black)),
            Field::Occupied(Piece::Knight(Color::Black)),
            Field::Occupied(Piece::Rook(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Occupied(Piece::Pawn(Color::Black)),
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Pawn(Color::White)),
            Field::Occupied(Piece::Rook(Color::White)),
            Field::Occupied(Piece::Knight(Color::White)),
            Field::Occupied(Piece::Bishop(Color::White)),
            Field::Occupied(Piece::Queen(Color::White)),
            Field::Occupied(Piece::King(Color::White)),
            Field::Occupied(Piece::Bishop(Color::White)),
            Field::Occupied(Piece::Knight(Color::White)),
            Field::Occupied(Piece::Rook(Color::White)),
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
