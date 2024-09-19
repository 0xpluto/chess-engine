
#[derive(Default, Copy, Clone, PartialEq)]
pub enum PieceType {
    #[default]
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn white(piece_type: PieceType) -> Self {
        Piece {
            piece_type,
            color: Color::White,
        }
    }
    pub fn black(piece_type: PieceType) -> Self {
        Piece {
            piece_type,
            color: Color::Black,
        }
    }
    pub fn empty() -> Self {
        Piece {
            piece_type: PieceType::Empty,
            color: Color::White,
        }
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PieceType::Empty => write!(f, " "),
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "♕"),
            PieceType::King => write!(f, "♔"),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::Empty => write!(f, " "),
                PieceType::Pawn => write!(f, "♙"),
                PieceType::Knight => write!(f, "♘"),
                PieceType::Bishop => write!(f, "♗"),
                PieceType::Rook => write!(f, "♖"),
                PieceType::Queen => write!(f, "♕"),
                PieceType::King => write!(f, "♔"),
            },
            Color::Black => match self.piece_type {
                PieceType::Empty => write!(f, " "),
                PieceType::Pawn => write!(f, "♟"),
                PieceType::Knight => write!(f, "♞"),
                PieceType::Bishop => write!(f, "♝"),
                PieceType::Rook => write!(f, "♜"),
                PieceType::Queen => write!(f, "♛"),
                PieceType::King => write!(f, "♚"),
            },
        }
    }
}
/*
Pawn

 *
_|_

Knight

 &
/ |
|_|

Bishop

 ^
/ |
|_|

Rook

|#|
|_|

Queen

 $
/ \
|_|

King

 #
/|\
|_|

*/