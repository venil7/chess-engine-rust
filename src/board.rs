use crate::field::{extract_piece, field_color, Field};
use crate::piece::{possible_moves, Color, Piece, PiecePosition};
use crate::position::{Move, Position};
use std::fmt;
use std::ops;

// type PiecePosition = (Piece, Position);
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
    // pub fn at(&self, position: &Position) -> Field {
    //     self.fields[position.to_index()]
    // }
    pub fn occupied_fields(&self, color: &Color) -> Vec<PiecePosition> {
        self.fields
            .iter()
            .zip(0..LENGTH)
            .filter(|(field, _)| {
                if let Some(color_) = field_color(field) {
                    color_ == *color
                } else {
                    false
                }
            })
            .map(|(field, index)| (extract_piece(field), Position::from_index(index)))
            .collect()
    }

    pub fn possible_moves(&self, color: &Color) -> Vec<Move> {
        self.occupied_fields(color)
            .iter()
            .flat_map(|pp| possible_moves(pp, self))
            .collect()
    }
}

impl ops::Index<Position> for Board {
    type Output = Field;
    fn index<'a>(&'a self, position: Position) -> &'a Field {
        &self.fields[position.to_index()]
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
                    result += "\n"
                }
                result
            });
        write!(formatter, "{}", str)
    }
}
