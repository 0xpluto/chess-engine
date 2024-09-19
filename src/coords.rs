use crate::pieces::{Piece, PieceType};


pub struct MoveCoords {
    pub piece: Piece,
    pub from: Coords,
    pub to: Coords,
    pub takes: bool,
    pub promotion: Option<PieceType>,
}

impl MoveCoords {
    pub fn promote(&self, piece: PieceType) -> Self {
        MoveCoords {
            piece: self.piece,
            from: self.from,
            to: self.to,
            takes: self.takes,
            promotion: Some(piece),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coords {
    pub x: u8,
    pub y: u8,
}

impl Coords {
    pub fn new(x: u8, y: u8) -> Self {
        Coords { x, y }
    }
    pub fn std(&self) -> (char, u8) {
        let x = (self.x as u8 + 97) as char;
        let y = 8 - self.y;
        (x, y)
    }
}

impl std::str::FromStr for Coords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let x = match chars.next() {
            Some(c) => c,
            None => return Err("No input"),
        };
        let y = match chars.next() {
            Some(c) => c,
            None => return Err("No input"),
        };
        let x = x as u8 - 97;
        let y = 8 - y.to_digit(10).unwrap() as u8;
        Ok(Coords::new(x, y))
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (x, y) = self.std();
        write!(f, "{}{}", x, y)
    }
}

impl From<(u8, u8)> for Coords {
    fn from((x, y): (u8, u8)) -> Self {
        Coords::new(x, y)
    }
}

impl std::fmt::Display for MoveCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (x, y) = self.to.std();
        if PieceType::Pawn == self.piece.piece_type && !self.takes {
            return write!(f, "{}{}", x, y);
        }
        let start = if PieceType::Pawn == self.piece.piece_type {
            self.from.std().0
        } else {
            self.piece.to_string().chars().next().unwrap()
        };
        let takes = if self.takes { "x" } else { "" };

        write!(f, "{}{}{}{}", start, takes, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_coords() {
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Pawn),
            from: "c2".parse().unwrap(),
            to: "c4".parse().unwrap(),
            takes: false,
            promotion: None,
        };
        assert!(mc.to_string() == "c4");
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Pawn),
            from: "c2".parse().unwrap(),
            to: "d3".parse().unwrap(),
            takes: true,
            promotion: None,
        };
        assert!(mc.to_string() == "cxd3");
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Rook),
            from: "a1".parse().unwrap(),
            to: "a8".parse().unwrap(),
            takes: true,
            promotion: None,
        };
        assert!(mc.to_string() == "♖xa8");
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Rook),
            from: "a1".parse().unwrap(),
            to: "a8".parse().unwrap(),
            takes: false,
            promotion: None,
        };
        assert!(mc.to_string() == "♖a8");
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Rook),
            from: "a1".parse().unwrap(),
            to: "h1".parse().unwrap(),
            takes: true,
            promotion: None,
        };
        assert!(mc.to_string() == "♖xh1");
        let mc = MoveCoords {
            piece: Piece::white(PieceType::Queen),
            from: "a1".parse().unwrap(),
            to: "h1".parse().unwrap(),
            takes: false,
            promotion: None,
        };
        assert!(mc.to_string() == "♕h1");
    }

    #[test]
    fn test_coords_std() {
        assert_eq!(Coords::new(0, 0).std(), ('a', 8));
        assert_eq!(Coords::new(7, 7).std(), ('h', 1));
    }

    #[test]
    fn test_coords_from_str() {
        assert_eq!("a8".parse(), Ok(Coords::new(0, 0)));
        assert_eq!("a7".parse(), Ok(Coords::new(0, 1)));
        assert_eq!("a6".parse(), Ok(Coords::new(0, 2)));
        assert_eq!("a5".parse(), Ok(Coords::new(0, 3)));
        assert_eq!("a4".parse(), Ok(Coords::new(0, 4)));
        assert_eq!("a3".parse(), Ok(Coords::new(0, 5)));
        assert_eq!("a2".parse(), Ok(Coords::new(0, 6)));
        assert_eq!("a1".parse(), Ok(Coords::new(0, 7)));
        assert_eq!("b8".parse(), Ok(Coords::new(1, 0)));
        assert_eq!("b7".parse(), Ok(Coords::new(1, 1)));
        assert_eq!("b6".parse(), Ok(Coords::new(1, 2)));
        assert_eq!("b5".parse(), Ok(Coords::new(1, 3)));
        assert_eq!("b4".parse(), Ok(Coords::new(1, 4)));
        assert_eq!("b3".parse(), Ok(Coords::new(1, 5)));
        assert_eq!("b2".parse(), Ok(Coords::new(1, 6)));
        assert_eq!("b1".parse(), Ok(Coords::new(1, 7)));
        assert_eq!("c8".parse(), Ok(Coords::new(2, 0)));
        assert_eq!("c7".parse(), Ok(Coords::new(2, 1)));
        assert_eq!("c6".parse(), Ok(Coords::new(2, 2)));
        assert_eq!("c5".parse(), Ok(Coords::new(2, 3)));
        assert_eq!("c4".parse(), Ok(Coords::new(2, 4)));
        assert_eq!("c3".parse(), Ok(Coords::new(2, 5)));
        assert_eq!("c2".parse(), Ok(Coords::new(2, 6)));
        assert_eq!("c1".parse(), Ok(Coords::new(2, 7)));

        assert_eq!("h1".parse(), Ok(Coords::new(7, 7)));
    }
}