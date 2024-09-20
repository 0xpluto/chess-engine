use crate::{
    board::Board,
    coords::{Coords, MoveCoords},
    pieces::{Color, Piece, PieceType},
};

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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                ..Default::default()
            });
        }
        // Take diagonally to the left
        if x > 0 && !self.is_empty(Coords::new(x - 1, y)) {
            moves.push(MoveCoords {
                piece,
                from: pawn_start,
                to: Coords::new(x - 1, y),
                takes: true,
                ..Default::default()
            });
        }
        // Take diagonally to the right
        if x < 7 && !self.is_empty(Coords::new(x + 1, y)) {
            moves.push(MoveCoords {
                piece,
                from: pawn_start,
                to: Coords::new(x + 1, y),
                takes: true,
                ..Default::default()
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
                        ..Default::default()
                    });
                } else if self.piece_color(mv) != self.piece_color(rook_start) {
                    moves.push(MoveCoords {
                        piece,
                        from: rook_start,
                        to: mv,
                        takes: true,
                        ..Default::default()
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
                        ..Default::default()
                    });
                } else if self.piece_color(mv) != self.piece_color(bishop_start) {
                    moves.push(MoveCoords {
                        piece,
                        from: bishop_start,
                        to: mv,
                        takes: true,
                        ..Default::default()
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
    pub fn knight_moves(&self, knight_start: Coords, color: Color) -> Vec<MoveCoords> {

        let mut moves = vec![];
        // 8 possible moves
        let x = knight_start.x as i8;
        let y = knight_start.y as i8;
        let piece = Piece {
            piece_type: PieceType::Knight,
            color,
        };
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
                    from: knight_start,
                    to: new_coord,
                    ..Default::default()
                });
            } else if self.piece_color(knight_start) != self.piece_color(new_coord) {
                moves.push(MoveCoords {
                    piece,
                    from: knight_start,
                    to: new_coord,
                    takes: true,
                    ..Default::default()
                });
            }
        }
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
        moves
            .iter()
            .map(|mc| MoveCoords {
                piece,
                from: queen_start,
                to: mc.to,
                takes: mc.takes,
                promotion: mc.promotion,
                ..Default::default()
            })
            .collect()
    }
    pub fn king_moves(&self, king_start: Coords, color: Color) -> Vec<MoveCoords> {
        let x = king_start.x;
        let y = king_start.y;
        let piece = Piece {
            piece_type: PieceType::King,
            color,
        };
        let mut moves = vec![];
        for dx in -2i8..=2i8 {
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

                if dx == 2 && dy == 0 && self.can_castle(color, true) {
                    moves.push(MoveCoords {
                        piece,
                        from: king_start,
                        to: Coords::new(new_x, new_y),
                        king_side_castle: true,
                        ..Default::default()
                    });
                    continue;
                } else if dx == -2 && dy == 0 && self.can_castle(color, false) {
                    moves.push(MoveCoords {
                        piece,
                        from: king_start,
                        to: Coords::new(new_x, new_y),
                        queen_side_castle: true,
                        ..Default::default()
                    });
                    continue;
                }

                // TODO Check if the king is in check
                // TODO Check if the king is moving into check
                // TODO Check if the king is castling through check
                if self.is_empty(new_coord) {
                    moves.push(MoveCoords {
                        piece,
                        from: king_start,
                        to: new_coord,
                        ..Default::default()
                    });
                } else if self.piece_color(new_coord) != self.piece_color(king_start) {
                    moves.push(MoveCoords {
                        piece,
                        from: king_start,
                        to: new_coord,
                        takes: true,
                        ..Default::default()
                    });
                }
            }
        }
        moves
    }
}
/*
King moves: [MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: f1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }, MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: g1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }]
Moving piece
King moves: [MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: f1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }, MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: g1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }]
King moves: [MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: f1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }, MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: g1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }]
MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: g1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }
Piece present at from
King moves: [MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: f1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }, MoveCoords { piece: Piece { piece_type: King, color: White }, from: e1, to: g1, takes: false, promotion: None, king_side_castle: false, queen_side_castle: false }] */
