use crate::{
    coords::{Coords, MoveCoords, KING_SIDE_BLACK_ROOK, KING_SIDE_WHITE_ROOK, QUEEN_SIDE_BLACK_ROOK, QUEEN_SIDE_WHITE_ROOK},
    pieces::{Color, Piece, PieceType},
};

pub struct Board {
    // ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜
    // ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟
    //
    //
    //
    //
    //  ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙
    //  ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖
    /// White side is row 0 and 1, black side is row 6 and 7
    pub state: [[Piece; 8]; 8],
    pub turn: Color,
    pub moves: Vec<MoveCoords>,
}

impl Board {
    pub fn new() -> Self {
        use PieceType::*;
        let state = [
            [
                Piece::black(Rook),
                Piece::black(Knight),
                Piece::black(Bishop),
                Piece::black(Queen),
                Piece::black(King),
                Piece::black(Bishop),
                Piece::black(Knight),
                Piece::black(Rook),
            ],
            [Piece::black(Pawn); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::empty(); 8],
            [Piece::white(Pawn); 8],
            [
                Piece::white(Rook),
                Piece::white(Knight),
                Piece::white(Bishop),
                Piece::white(Queen),
                Piece::white(King),
                Piece::white(Bishop),
                Piece::white(Knight),
                Piece::white(Rook),
            ],
        ];
        Board {
            state,
            turn: Color::White,
            moves: vec![],
        }
    }
    pub fn piece_present(&self, piece: Piece, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x] == piece
    }
    pub fn can_promote(&self, from: impl Into<Coords>) -> bool {
        self.possible_moves(from).iter().any(|mc| mc.promotion.is_some())
    }
    pub fn is_promote(&self, from: impl Into<Coords>, to: impl Into<Coords>) -> bool {
        let to: Coords = to.into();
        self.possible_moves(from).iter().any(|mc| mc.to == to && mc.promotion.is_some())
    }
    fn can_take(&self, from: impl Into<Coords>, to: impl Into<Coords>) -> bool {
        let to: Coords = to.into();
        let moves = self.possible_moves(from);
        moves.iter().any(|mc| mc.to == to && mc.takes)
    }
    pub fn can_castle(&self, color: Color, king_side: bool) -> bool {
        if self.king_has_moved(color) {
            return false;
        }
        if self.rook_has_moved(color, king_side) {
            return false;
        }
        let y = match color {
            Color::White => 7,
            Color::Black => 0,
        };
        let x = match king_side {
            true => 7,
            false => 0,
        };
        let rook = self.state[y][x];
        if rook.piece_type != PieceType::Rook {
            // Should never happen
            println!("Rook not present but hasn't moved");
            return false;
        }
        let mut x = match king_side {
            true => 6,
            false => 1,
        };
        while x != 4 {
            if !self.is_empty(Coords::new(x, y as u8)) {
                return false;
            }
            x = match king_side {
                true => x - 1,
                false => x + 1,
            };
        }
        true
    }

    pub fn king_has_moved(&self, color: Color) -> bool {
        self.moves.iter().any(|mc| mc.piece.piece_type == PieceType::King && mc.piece.color == color)
    }

    pub fn rook_has_moved(&self, color: Color, king_side: bool) -> bool {
        match color {
            Color::White => {
                if king_side {
                    self.moves.iter().any(|mc| mc.piece.piece_type == PieceType::Rook && mc.piece.color == color && mc.from == KING_SIDE_WHITE_ROOK)
                } else {
                    self.moves.iter().any(|mc| mc.piece.piece_type == PieceType::Rook && mc.piece.color == color && mc.from == QUEEN_SIDE_WHITE_ROOK)
                }
            }
            Color::Black => {
                if king_side {
                    self.moves.iter().any(|mc| mc.piece.piece_type == PieceType::Rook && mc.piece.color == color && mc.from == KING_SIDE_BLACK_ROOK)
                } else {
                    self.moves.iter().any(|mc| mc.piece.piece_type == PieceType::Rook && mc.piece.color == color && mc.from == QUEEN_SIDE_BLACK_ROOK)
                }
            }
        }
    }

    pub fn get_piece(&self, coord: impl Into<Coords>) -> Piece {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x]
    }
    pub fn is_empty(&self, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x].piece_type == PieceType::Empty
    }
    pub fn movable_piece(&self, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x].color == self.turn
    }
    pub fn piece_color(&self, coord: impl Into<Coords>) -> Option<Color> {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        if self.is_empty(coord) {
            return None;
        }
        Some(self.state[y][x].color)
    }
    fn possible_moves(&self, coord: impl Into<Coords>) -> Vec<MoveCoords> {
        let coord: Coords = coord.into();
        let x = coord.x;
        let y = coord.y;
        let piece = self.state[y as usize][x as usize];
        match piece.piece_type {
            PieceType::Pawn => {
                self.pawn_moves(coord, piece.color)
            }
            PieceType::Bishop => {
                self.bishop_moves(coord, piece.color)
            }
            PieceType::Rook => {
                self.rook_moves(coord, piece.color)
            }
            PieceType::Queen => {
                self.queen_moves(coord, piece.color)
            }
            PieceType::King => {
                self.king_moves(coord, piece.color)
            }
            PieceType::Knight => {
                let mut moves = vec![];
                // 8 possible moves
                let x = x as i8;
                let y = y as i8;
                // 4 moves move 2 in x and 1 in y
                // 4 moves move 2 in y and 1 in x
                let possible_moves = vec![
                    (x + 2, y + 1),
                    (x + 2, y - 1),
                    (x - 2, y + 1),
                    (x - 2, y - 1),
                    (x + 1, y + 2),
                    (x + 1, y - 2),
                    (x - 1, y + 2),
                    (x - 1, y - 2),
                ];
                for (nx, ny) in possible_moves {
                    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
                        continue;
                    }
                    let new_coord = Coords::new(nx as u8, ny as u8);
                    if self.is_empty(new_coord) {
                        moves.push(MoveCoords {
                            piece,
                            from: coord,
                            to: new_coord,
                            ..Default::default()
                        });
                    } else if self.piece_color(coord) != self.piece_color(new_coord){
                        moves.push(MoveCoords {
                            piece,
                            from: coord,
                            to: new_coord,
                            takes: true,
                            ..Default::default()
                        });
                    }
                }
                moves
            }

            _ => vec![],
        }
    }
    fn valid_move(&self, mc: &MoveCoords) -> bool {
        // Check if the piece is present
        if !self.piece_present(mc.piece, mc.from) {
            return false;
        }
        println!("Piece present at from");
        // Check if it's the correct turn
        if mc.piece.color != self.turn {
            return false;
        }
        
        self.possible_moves(mc.from).iter().any(|m| m == mc)
    }

    fn parse_request(&self, from: impl Into<Coords>, to: impl Into<Coords>, promotion: Option<PieceType>) -> Option<MoveCoords> {
        let from: Coords = from.into();
        let to: Coords = to.into();
        let piece = self.get_piece(from);
        let dx = to.x as i8 - from.x as i8;
        let color = piece.color;
        if dx == 2 && self.can_castle(color, true) {
            return Some(MoveCoords {
                piece,
                from,
                to,
                takes: false,
                king_side_castle: true,
                queen_side_castle: false,
                promotion: None,
            });
        } else if dx == -2 && self.can_castle(color, false) {
            return Some(MoveCoords {
                piece,
                from,
                to,
                takes: false,
                king_side_castle: false,
                queen_side_castle: true,
                promotion: None,
            });
        }
        
        let takes = self.can_take(from, to);
        let promotion = if self.can_promote(from) {
            promotion
        } else {
            None
        };
        Some(MoveCoords {
            piece,
            from,
            to,
            takes,
            promotion,
            ..Default::default()
        })
    }

    fn toggle_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn move_piece_unchecked(&mut self, from: impl Into<Coords>, to: impl Into<Coords>, promotion: Option<PieceType>, castle_king_side: bool, castle_queen_side: bool) {
        let from: Coords = from.into();
        let mut from_piece = self.get_piece(from);
        let to: Coords = to.into();
        if let Some(promotion) = promotion {
            from_piece.piece_type = promotion;
        }
        match (from_piece.color, castle_king_side, castle_queen_side) {
            (Color::White, true, false) => {
                self.state[7][4] = Piece::empty();
                self.state[7][5] = Piece::white(PieceType::Rook);
                self.state[7][6] = Piece::white(PieceType::King);
                self.state[7][7] = Piece::empty();
            }
            (Color::White, false, true) => {
                self.state[7][0] = Piece::empty();
                self.state[7][1] = Piece::empty();
                self.state[7][2] = Piece::white(PieceType::King);
                self.state[7][3] = Piece::white(PieceType::Rook);
                self.state[7][4] = Piece::empty();
            }
            (Color::Black, true, false) => {
                self.state[0][4] = Piece::empty();
                self.state[0][5] = Piece::black(PieceType::Rook);
                self.state[0][6] = Piece::black(PieceType::King);
                self.state[0][7] = Piece::empty();
            }
            (Color::Black, false, true) => {
                self.state[0][0] = Piece::empty();
                self.state[0][1] = Piece::empty();
                self.state[0][2] = Piece::black(PieceType::King);
                self.state[0][3] = Piece::black(PieceType::Rook);
                self.state[0][4] = Piece::empty();
            },
            (_, false, false) => {
                self.state[to.y as usize][to.x as usize] = from_piece;
                self.state[from.y as usize][from.x as usize] = Piece::empty();
            }
            _ => (),
        }
        self.toggle_turn();
    }

    pub fn move_piece(&mut self, from: impl Into<Coords>, to: impl Into<Coords>, promotion: Option<PieceType>) -> bool {
        println!("Moving piece");
        let from: Coords = from.into();
        let to: Coords = to.into();
        let mc = match self.parse_request(from, to, promotion) {
            Some(mc) => mc,
            None => return false,
        };
        println!("{:?}", mc);
        if !self.valid_move(&mc) {
            return false;
        }
        self.move_piece_unchecked(from, to, promotion, mc.king_side_castle, mc.queen_side_castle);
        self.moves.push(mc);
        true
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
    fn test_valid_moves() {
        let board = Board::new();
        let moves = board.possible_moves("a2".parse::<Coords>().unwrap());
        assert!(moves.len() == 2);
        assert!(moves
            .iter()
            .any(|mc| mc.to == "a3".parse::<Coords>().unwrap()));
        assert!(moves
            .iter()
            .any(|mc| mc.to == "a4".parse::<Coords>().unwrap()));
        let moves = board.possible_moves("a7".parse::<Coords>().unwrap());
        assert!(moves.len() == 2);
        assert!(moves
            .iter()
            .any(|mc| mc.to == "a6".parse::<Coords>().unwrap()));
        assert!(moves
            .iter()
            .any(|mc| mc.to == "a5".parse::<Coords>().unwrap()));
        let moves = board.possible_moves("b1".parse::<Coords>().unwrap());
        for mv in moves.iter() {
            println!("{}", mv);
        }
        assert!(moves.len() == 2);
        assert!(moves
            .iter()
            .any(|mc| mc.to == "a3".parse::<Coords>().unwrap()));
        assert!(moves
            .iter()
            .any(|mc| mc.to == "c3".parse::<Coords>().unwrap()));
        
    }

    #[test]
    fn test_piece_present() {
        let board = Board::new();
        assert!(board.piece_present(
            Piece::black(PieceType::Rook),
            "a8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Knight),
            "b8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Bishop),
            "c8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Queen),
            "d8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::King),
            "e8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Bishop),
            "f8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Knight),
            "g8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Rook),
            "h8".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "a7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "b7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "c7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "d7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "e7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "f7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "g7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::black(PieceType::Pawn),
            "h7".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "a2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "b2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "c2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "d2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "e2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "f2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "g2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Pawn),
            "h2".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Rook),
            "a1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Knight),
            "b1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Bishop),
            "c1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Queen),
            "d1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::King),
            "e1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Bishop),
            "f1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Knight),
            "g1".parse::<Coords>().unwrap()
        ));
        assert!(board.piece_present(
            Piece::white(PieceType::Rook),
            "h1".parse::<Coords>().unwrap()
        ));
    }
}
