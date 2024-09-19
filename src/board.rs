use crate::{
    coords::{Coords, MoveCoords},
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
    state: [[Piece; 8]; 8],
    turn: Color,
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
        }
    }
    fn piece_present(&self, piece: Piece, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x] == piece
    }
    fn get_piece(&self, coord: impl Into<Coords>) -> Piece {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x]
    }
    fn is_empty(&self, coord: impl Into<Coords>) -> bool {
        let coord: Coords = coord.into();
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.state[y][x].piece_type == PieceType::Empty
    }
    fn piece_color(&self, coord: impl Into<Coords>) -> Option<Color> {
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
                let mut moves = vec![];
                let y = match piece.color {
                    Color::White => {
                        // Promotion
                        if y == 1 && self.is_empty((x, 0)) {
                            use PieceType::*;
                            let mc = MoveCoords {
                                piece,
                                from: coord,
                                to: Coords::new(x, 0),
                                takes: false,
                                promotion: None,
                            };
                            moves.extend(vec![
                                mc.promote(Queen),
                                mc.promote(Rook),
                                mc.promote(Bishop),
                                mc.promote(Knight),
                            ]);

                        // Starting move 2 squares
                        } else if y == 6 && self.is_empty((x, 5)) && self.is_empty((x, 4)) {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: Coords::new(x, 4),
                                takes: false,
                                promotion: None,
                            });
                        }
                        y - 1
                    }
                    Color::Black => {
                        // Promotion
                        if y == 6 && self.is_empty((x, 7)) {
                            use PieceType::*;
                            let mc = MoveCoords {
                                piece,
                                from: coord,
                                to: Coords::new(x, 5),
                                takes: false,
                                promotion: None,
                            };
                            moves.extend(vec![
                                mc.promote(Queen),
                                mc.promote(Rook),
                                mc.promote(Bishop),
                                mc.promote(Knight),
                            ]);

                        // Starting move 2 squares
                        } else if y == 1 && self.is_empty((x, 2)) && self.is_empty((x, 3)) {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: Coords::new(x, 3),
                                takes: false,
                                promotion: None,
                            });
                        }
                        y + 1
                    }
                };
                // Advance normally
                if self.is_empty((x, y)) {
                    moves.push(MoveCoords {
                        piece,
                        from: coord,
                        to: Coords::new(x, y),
                        takes: false,
                        promotion: None,
                    });
                }
                // Take diagonally to the left
                if x > 0 && !self.is_empty(Coords::new(x - 1, y)) {
                    moves.push(MoveCoords {
                        piece,
                        from: coord,
                        to: Coords::new(x - 1, y),
                        takes: true,
                        promotion: None,
                    });
                }
                // Take diagonally to the right
                if x < 7 && !self.is_empty(Coords::new(x + 1, y)) {
                    moves.push(MoveCoords {
                        piece,
                        from: coord,
                        to: Coords::new(x + 1, y),
                        takes: true,
                        promotion: None,
                    });
                }

                moves
            }
            PieceType::Bishop => {
                let mut moves = vec![];
                let x = x as i8;
                let y = y as i8;

                let mut top_left = vec![];
                let mut top_right = vec![];
                let mut bottom_left = vec![];
                let mut bottom_right = vec![];
                for i in 1..8 {
                    top_left.push((x - i, y - i));
                    top_right.push((x + i, y - i));
                    bottom_left.push((x - i, y + i));
                    bottom_right.push((x + i, y + i));
                }
                let parse_diag = |mut diag: Vec<(i8, i8)>, moves: &mut Vec<MoveCoords>| {
                    diag.retain(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8);
                    let diag = diag.iter().map(|(x, y)| Coords::new(*x as u8, *y as u8));
                    for mv in diag {
                        if self.is_empty(mv) {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: mv,
                                takes: false,
                                promotion: None,
                            });
                        } else if self.piece_color(mv) != self.piece_color(coord) {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: mv,
                                takes: true,
                                promotion: None,
                            });
                            break;
                        } else {
                            break;
                        }
                    }
                };
                parse_diag(top_left, &mut moves);
                parse_diag(top_right, &mut moves);
                parse_diag(bottom_left, &mut moves);
                parse_diag(bottom_right, &mut moves);

                moves
            }
            PieceType::King => {
                let mut moves = vec![];
                for dx in -1i8..=1i8 {
                    for dy in -1i8..=1i8 {
                        // Not moving
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        // Off the board x axis
                        if x == 0 && dx == -1 || x == 7 && dx == 1 {
                            continue;
                        }
                        // Off the board y axis
                        if y == 0 && dy == -1 || y == 7 && dy == 1 {
                            continue;
                        }
                        let new_x = (x as i8 + dx) as u8;
                        let new_y = (y as i8 + dy) as u8;
                        let new_coord = Coords::new(new_x, new_y);

                        // TODO Check if the king is in check
                        // TODO Check if the king is moving into check
                        // TODO Check if the king is castling
                        // TODO Check if the king is castling through check
                        if self.is_empty(new_coord) {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: new_coord,
                                takes: false,
                                promotion: None,
                            });
                        } else {
                            moves.push(MoveCoords {
                                piece,
                                from: coord,
                                to: new_coord,
                                takes: true,
                                promotion: None,
                            });
                        }
                    }
                }
                moves
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
                            takes: false,
                            promotion: None,
                        });
                    } else if self.piece_color(coord) != self.piece_color(new_coord){
                        moves.push(MoveCoords {
                            piece,
                            from: coord,
                            to: new_coord,
                            takes: true,
                            promotion: None,
                        });
                    }
                }
                moves
            }

            _ => vec![],
        }
    }
    fn valid_move(&self, mc: MoveCoords) -> bool {
        // Check if the piece is present
        if !self.piece_present(mc.piece, mc.from) {
            return false;
        }
        // Check if it's the correct turn
        if mc.piece.color != self.turn {
            return false;
        }
        match mc.piece.piece_type {
            PieceType::Pawn => {}
            _ => {}
        }

        false
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

        let moves = board.possible_moves("c1".parse::<Coords>().unwrap());
        
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
