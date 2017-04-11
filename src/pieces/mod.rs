use std::fmt;

pub mod pawn;

#[derive(PartialEq,Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

pub trait Piece: fmt::Display {
    fn get_type(&self) -> PieceType;
}