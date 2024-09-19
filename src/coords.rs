use crate::pieces::{Piece, PieceType};


pub struct MoveCoords {
    piece: PieceType,
    from: Coords,
    to: Coords,
    takes: bool,
}

#[derive(PartialEq, Debug)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }
    pub fn std(&self) -> (char, usize) {
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
        let x = x as usize - 97;
        let y = 8 - y.to_digit(10).unwrap() as usize;
        Ok(Coords::new(x, y))
    }
}

impl From<(usize, usize)> for Coords {
    fn from((x, y): (usize, usize)) -> Self {
        Coords::new(x, y)
    }
}

impl std::fmt::Display for MoveCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (x, y) = self.to.std();
        let start = if PieceType::Pawn == self.piece {
            x
        } else {
            self.piece.to_string().chars().next().unwrap()
        };
        let takes = if self.takes { "x" } else { "" };
        write!(f, "{}{}{}{}", start, y, takes, self.to.std())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_coords() {
        let mc = MoveCoords {
            piece: PieceType::Pawn,
            from: "c2".parse().unwrap(),
            to: "c4".parse().unwrap(),
            takes: false,
        };
        println!("{}", mc.to_string());
        let mc = MoveCoords {
            piece: PieceType::Pawn,
            from: "c2".parse().unwrap(),
            to: "d3".parse().unwrap(),
            takes: true,
        };
        println!("{}", mc.to_string());
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