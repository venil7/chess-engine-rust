pub mod pawn;
pub enum Pieces {
    Empty,
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

pub trait Piece {}
pub struct Empty;
impl Piece for Empty {}