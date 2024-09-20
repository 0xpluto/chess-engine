use crate::{board::Board, coords::{Coords, MoveCoords}, pieces::{Color, Piece, PieceType}};


impl Board {
    pub fn pawn_moves(&self, pawn_start: Coords, color: Color) -> Vec<MoveCoords> {
        let x = pawn_start.x;
        let y = pawn_start.y;
        let piece = Piece {
            piece_type: PieceType::Pawn,
            color,
        };
        let mut moves = vec![];
                let y = match piece.color {
                    Color::White => {
                        // Promotion
                        if y == 1 && self.is_empty((x, 0)) {
                            use PieceType::*;
                            let mc = MoveCoords {
                                piece,
                                from: pawn_start,
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
                                from: pawn_start,
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
                                from: pawn_start,
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
                                from: pawn_start,
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
                        from: pawn_start,
                        to: Coords::new(x, y),
                        takes: false,
                        promotion: None,
                    });
                }
                // Take diagonally to the left
                if x > 0 && !self.is_empty(Coords::new(x - 1, y)) {
                    moves.push(MoveCoords {
                        piece,
                        from: pawn_start,
                        to: Coords::new(x - 1, y),
                        takes: true,
                        promotion: None,
                    });
                }
                // Take diagonally to the right
                if x < 7 && !self.is_empty(Coords::new(x + 1, y)) {
                    moves.push(MoveCoords {
                        piece,
                        from: pawn_start,
                        to: Coords::new(x + 1, y),
                        takes: true,
                        promotion: None,
                    });
                }

                moves
    }
    pub fn rook_moves(&self, rook_start: Coords, color: Color) -> Vec<MoveCoords> {
        let x = rook_start.x;
        let y = rook_start.y;
        let piece = Piece {
            piece_type: PieceType::Rook,
            color,
        };
        let mut moves = vec![];
                let x = x as i8;
                let y = y as i8;

                let mut up = vec![];
                let mut down = vec![];
                let mut left = vec![];
                let mut right = vec![];
                for i in 1..8 {
                    up.push((x, y - i));
                    down.push((x, y + i));
                    left.push((x - i, y));
                    right.push((x + i, y));
                }
                let parse_line = |mut line: Vec<(i8, i8)>, moves: &mut Vec<MoveCoords>| {
                    line.retain(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8);
                    let line = line.iter().map(|(x, y)| Coords::new(*x as u8, *y as u8));
                    for mv in line {
                        if self.is_empty(mv) {
                            moves.push(MoveCoords {
                                piece,
                                from: rook_start,
                                to: mv,
                                takes: false,
                                promotion: None,
                            });
                        } else if self.piece_color(mv) != self.piece_color(rook_start) {
                            moves.push(MoveCoords {
                                piece,
                                from: rook_start,
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
                parse_line(up, &mut moves);
                parse_line(down, &mut moves);
                parse_line(left, &mut moves);
                parse_line(right, &mut moves);

                moves
    }

    pub fn bishop_moves(&self, bishop_start: Coords, color: Color) -> Vec<MoveCoords> {
        let x = bishop_start.x;
        let y = bishop_start.y;
        let piece = Piece {
            piece_type: PieceType::Bishop,
            color,
        };
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
                                from: bishop_start,
                                to: mv,
                                takes: false,
                                promotion: None,
                            });
                        } else if self.piece_color(mv) != self.piece_color(bishop_start) {
                            moves.push(MoveCoords {
                                piece,
                                from: bishop_start,
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

    pub fn queen_moves(&self, queen_start: Coords, color: Color) -> Vec<MoveCoords> {
        let mut moves = vec![];
        let piece = Piece {
            piece_type: PieceType::Queen,
            color,
        };
        moves.append(&mut self.rook_moves(queen_start, color));
        moves.append(&mut self.bishop_moves(queen_start, color));
        moves.iter().map(|mc| MoveCoords {
            piece,
            from: queen_start,
            to: mc.to,
            takes: mc.takes,
            promotion: mc.promotion,
        }).collect()
    }
}