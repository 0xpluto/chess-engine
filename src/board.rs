use crate::{coords::Coords, pieces::{Color, Piece, PieceType}};


pub struct Board {
    /// White side is row 0 and 1, black side is row 6 and 7
    state: [[Piece; 8]; 8],
    turn: Color,
}

impl Board {
    pub fn new() -> Self {
        use PieceType::*;
        let state = [
            [Piece::black(Rook), Piece::black(Knight), Piece::black(Bishop), Piece::black(Queen), Piece::black(King), Piece::black(Bishop), Piece::black(Knight), Piece::black(Rook)],
            [Piece::black(Pawn); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::white(Pawn); 8],
            [Piece::white(Rook), Piece::white(Knight), Piece::white(Bishop), Piece::white(Queen), Piece::white(King), Piece::white(Bishop), Piece::white(Knight), Piece::white(Rook)],
        ];
        Board { state, turn: Color::White }
    }
    fn piece_present(&self, piece: Piece, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x;
        let y = coord.y;
        self.state[y][x] == piece
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.state.iter() {
            for piece in row.iter() {
                write!(f, " {} ", piece)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coords;

    use super::*;
    /*
    |bR||bN||bB||bQ||bK||bB||bN||bR|
    |bP||bP||bP||bP||bP||bP||bP||bP|
    |  ||  ||  ||  ||  ||  ||  ||  |
    |  ||  ||  ||  ||  ||  ||  ||  |
    |  ||  ||  ||  ||  ||  ||  ||  |
    |  ||  ||  ||  ||  ||  ||  ||  |
    |wP||wP||wP||wP||wP||wP||wP||wP|
    |wR||wN||wB||wQ||wK||wB||wN||wR|
     */

    #[test]
    fn test_board_display() {
        let board = Board::new();
        println!("{}", board);
    }

    #[test]
    fn test_piece_present() {
        let board = Board::new();
        assert!(board.piece_present(Piece::black(PieceType::Rook), "a8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Knight), "b8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Bishop), "c8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Queen), "d8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::King), "e8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Bishop), "f8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Knight), "g8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Rook), "h8".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "a7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "b7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "c7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "d7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "e7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "f7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "g7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::black(PieceType::Pawn), "h7".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "a2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "b2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "c2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "d2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "e2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "f2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "g2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Pawn), "h2".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Rook), "a1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Knight), "b1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Bishop), "c1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Queen), "d1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::King), "e1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Bishop), "f1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Knight), "g1".parse::<Coords>().unwrap()));
        assert!(board.piece_present(Piece::white(PieceType::Rook), "h1".parse::<Coords>().unwrap()));
    }
}